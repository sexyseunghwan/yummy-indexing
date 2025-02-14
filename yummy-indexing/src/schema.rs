// @generated automatically by Diesel CLI.

diesel::table! {
    migrations (id) {
        id -> Integer,
        timestamp -> Bigint,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    recommend_tbl (recommend_seq) {
        recommend_seq -> Integer,
        #[max_length = 255]
        recommend_name -> Varchar,
        #[max_length = 1]
        recommend_yn -> Char,
        reg_dt -> Datetime,
        chg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        reg_id -> Varchar,
        #[max_length = 25]
        chg_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    store (seq) {
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        #[max_length = 30]
        type_ -> Nullable<Varchar>,
        #[max_length = 500]
        address -> Nullable<Varchar>,
        lat -> Decimal,
        lng -> Decimal,
        reg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        reg_id -> Nullable<Varchar>,
        chg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        chg_id -> Nullable<Varchar>,
        seq -> Integer,
    }
}

diesel::table! {
    store_backup (seq) {
        seq -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        #[max_length = 30]
        type_ -> Nullable<Varchar>,
        #[max_length = 500]
        address -> Nullable<Varchar>,
        lat -> Decimal,
        lng -> Decimal,
        reg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        reg_id -> Nullable<Varchar>,
        chg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        chg_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    store_recommend_tbl (recommend_seq, seq) {
        recommend_seq -> Integer,
        seq -> Integer,
        recommend_end_dt -> Datetime,
        reg_dt -> Datetime,
        chg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        reg_id -> Varchar,
        #[max_length = 25]
        chg_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    zero_possible_market (seq) {
        seq -> Integer,
        #[max_length = 1]
        use_yn -> Char,
        #[max_length = 255]
        name -> Varchar,
        reg_dt -> Nullable<Datetime>,
        chg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        reg_id -> Nullable<Varchar>,
        #[max_length = 25]
        chg_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    zero_possible_market_backup (seq) {
        seq -> Integer,
        #[max_length = 1]
        use_yn -> Char,
        #[max_length = 255]
        name -> Varchar,
        reg_dt -> Nullable<Datetime>,
        chg_dt -> Nullable<Datetime>,
        #[max_length = 25]
        reg_id -> Nullable<Varchar>,
        #[max_length = 25]
        chg_id -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    migrations,
    recommend_tbl,
    store,
    store_backup,
    store_recommend_tbl,
    zero_possible_market,
    zero_possible_market_backup,
);
