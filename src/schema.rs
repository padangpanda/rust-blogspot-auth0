table! {
    accounts (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        account_type -> Text,
        tenant_domain -> Text,
        region -> Text,
        environment_tag -> Text,
        provider -> Text,
        created_at -> Timestamp,
    }
}

table! {
    apis (id) {
        id -> Int4,
        name -> Text,
        api_id -> Text,
        identifier -> Text,
        token_exp -> Int4,
        token_exp_browser -> Int4,
        sign_algorithm -> Text,
        rbac -> Bool,
        permission_acc_token -> Bool,
        allow_skip_user -> Bool,
        allow_off_acc -> Bool,
        account_id -> Int4,
    }
}

table! {
    applications (id) {
        id -> Int4,
        name -> Text,
        client_id -> Text,
        app_type -> Text,
        domain -> Text,
        client_secret -> Text,
        description -> Text,
        logo_url -> Text,
        token_auth_method -> Text,
        app_login_url -> Nullable<Text>,
        callback_url -> Nullable<Text>,
        logout_url -> Nullable<Text>,
        web_origin -> Nullable<Text>,
        cors -> Nullable<Text>,
        id_token_exp -> Int4,
        reuse_interval -> Int4,
        abs_lifetime -> Int4,
        inactivity_lifetime -> Int4,
        account_id -> Int4,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        account_id -> Int4,
    }
}

table! {
    settings (id) {
        id -> Int4,
        friendly_name -> Text,
        logo_url -> Text,
        support_email -> Text,
        support_url -> Text,
        environment_tag -> Text,
        default_audience -> Text,
        default_directory -> Text,
        default_error_page -> Text,
        default_error_page_url -> Text,
        default_language -> Text,
        account_id -> Int4,
    }
}

table! {
    user_role (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        latest_login -> Nullable<Timestamp>,
        connection -> Text,
        provider -> Text,
        is_social -> Bool,
        picture -> Text,
        updated_at -> Timestamp,
        blocked -> Bool,
        blocked_for -> Nullable<Text>,
        guardian_authenticators -> Nullable<Text>,
        account_id -> Int4,
    }
}

joinable!(apis -> accounts (account_id));
joinable!(applications -> accounts (account_id));
joinable!(roles -> accounts (account_id));
joinable!(settings -> accounts (account_id));
joinable!(user_role -> roles (role_id));
joinable!(user_role -> users (user_id));
joinable!(users -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    apis,
    applications,
    roles,
    settings,
    user_role,
    users,
);
