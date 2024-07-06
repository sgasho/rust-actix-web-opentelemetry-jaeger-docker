use crate::model::{UserWithTeam, Team};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use tracing::{Level, span, Span};
use uuid::Uuid;

#[post("/test-data")]
#[tracing::instrument()]
async fn insert_test_data(db: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let insert_user_span: Span = span!(Level::INFO, "INSERT IGNORE INTO user (user_id, name, email) VALUES (?, ?, ?)");
    let user_id = Uuid::new_v4().to_string();
    {
        let _enter = insert_user_span.enter();
        match sqlx::query(r#"INSERT IGNORE INTO user (user_id, name, email) VALUES (?, ?, ?)"#)
            .bind(&user_id)
            .bind("user1")
            .bind("abc@abc.efg")
            .execute(&**db)
            .await
        {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"status": "error","message": format!("{:?}", e)}));
            }
            Ok(_) => {}
        };
    }

    let insert_team_span: Span = span!(Level::INFO, "INSERT IGNORE INTO team (team_id, name) VALUES (?, ?)");
    let team_id = Uuid::new_v4().to_string();
    {
        let _enter = insert_team_span.enter();
        match sqlx::query(r#"INSERT IGNORE INTO team (team_id, name) VALUES (?, ?)"#)
            .bind(&team_id)
            .bind("team_1")
            .execute(&**db)
            .await
        {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"status": "error","message": format!("{:?}", e)}));
            }
            Ok(_) => {}
        };
    }

    let insert_team_member_span: Span = span!(Level::INFO, "INSERT IGNORE INTO team_member (team_id, user_id, member_rank) VALUES (?, ?, ?)");
    {
        let _enter = insert_team_member_span.enter();
        match sqlx::query(r#"INSERT IGNORE INTO team_member (team_id, user_id, member_rank) VALUES (?, ?, ?)"#)
            .bind(&team_id)
            .bind(&user_id)
            .bind(1)
            .execute(&**db)
            .await
        {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"status": "error","message": format!("{:?}", e)}));
            }
            Ok(_) => {}
        };
    }

    let get_user_span: Span = span!(Level::INFO, "SELECT u.user_id, u.name, u.email, tm.team_id, u.created_at, u.updated_at FROM user u JOIN team_member tm ON u.user_id = tm.user_id WHERE u.user_id = ?");
    let user_result: UserWithTeam = {
        let _enter = get_user_span.enter();
        match sqlx::query_as(
            r#"SELECT u.user_id, u.name, u.email, tm.team_id, u.created_at, u.updated_at FROM user u JOIN team_member tm ON u.user_id = tm.user_id WHERE u.user_id = ?"#,
        )
            .bind(&user_id)
            .fetch_one(&**db)
            .await
        {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"status": "error","message": format!("{:?}", e)}));
            }
            Ok(res) => res,
        }
    };

    let get_team_span: Span = span!(Level::INFO, "SELECT u.user_id, u.name, u.email, tm.team_id, u.created_at, u.updated_at FROM user u JOIN team_member tm ON u.user_id = tm.user_id WHERE u.user_id = ?");
    let team_result: Team = {
        let _enter = get_team_span.enter();
        match sqlx::query_as(r#"SELECT team_id, name, created_at, updated_at FROM team WHERE team_id = ?"#)
            .bind(&team_id)
            .fetch_one(&**db)
            .await
        {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"status": "error","message": format!("{:?}", e)}));
            }
            Ok(res) => res,
        }
    };

    HttpResponse::Ok().json(json!({
        "user": {
            "user_id": &user_result.user_id,
            "name": &user_result.name,
            "email": &user_result.email,
            "team_id": &user_result.team_id,
            "created_at": &user_result.created_at,
            "updated_at": &user_result.updated_at
        },
        "team": {
            "team_id": &team_result.team_id,
            "name": &team_result.name,
            "created_at": &team_result.created_at,
            "updated_at": &team_result.updated_at
        }
    }))
}

#[get("/users/{user_id}")]
#[tracing::instrument()]
async fn get_user(path: web::Path<Uuid>, db: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let user_id = path.into_inner().to_string();

    let get_user_span: Span = span!(Level::INFO, "SELECT u.user_id, u.name, u.email, tm.team_id, u.created_at, u.updated_at FROM user u JOIN team_member tm ON u.user_id = tm.user_id WHERE u.user_id = ?");
    let user: UserWithTeam = {
        let _enter = get_user_span.enter();
        match sqlx::query_as(
        r#"SELECT u.user_id, u.name, u.email, tm.team_id, u.created_at, u.updated_at FROM user u JOIN team_member tm ON u.user_id = tm.user_id WHERE u.user_id = ?"#,
    )
            .bind(&user_id)
            .fetch_one(&**db)
            .await
        {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"status": "error","message": format!("{:?}", e)}));
            }
            Ok(res) => res,
        }
    };

    HttpResponse::Ok().json(json!({
        "user": {
            "user_id": &user.user_id,
            "name": &user.name,
            "email": &user.email,
            "team_id": &user.team_id,
            "created_at": &user.created_at,
            "updated_at": &user.updated_at
        },
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .service(insert_test_data)
        .service(get_user));
}
