table! {
    gallery (message_id) {
        message_id -> Integer,
        gallery_id -> Integer,
        token -> Text,
        title -> Text,
        tags -> Text,
        telegraph -> Text,
        upload_images -> SmallInt,
        publish_date -> Date,
        poll_id -> Text,
        score -> Float,
        votes -> Text,
    }
}

table! {
    image_hash (hash) {
        hash -> Text,
        url -> Text,
    }
}

table! {
    images (fileindex) {
        fileindex -> Integer,
        url -> Text,
    }
}

table! {
    user_vote (user_id, poll_id) {
        user_id -> BigInt,
        poll_id -> Integer,
        option -> Integer,
        vote_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(gallery, image_hash, images, user_vote,);
