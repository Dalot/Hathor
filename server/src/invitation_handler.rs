use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::services::email as EmailService;
use crate::errors::service::ServiceError;
use crate::models::invitation::Invitation;
use crate::database::types::{Pool};

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // run diesel blocking code. Diesel queries are not async so we need to use web::block() helper to run sync code and still return a future by the parent method.
    let res = web::block(move || create_invitation(invitation_data.into_inner().email, pool)).await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn create_invitation(
    email: String,
    pool: web::Data<Pool>,
) -> Result<(), ServiceError> {
    let invitation = dbg!(query(email, pool)?);
    EmailService::send_invitation(&invitation)
}

/// Diesel query
fn query(email: String, pool: web::Data<Pool>) -> Result<Invitation, ServiceError> {
    use crate::database::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = email.into();
    let conn: &PgConnection = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}