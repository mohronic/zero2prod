-- Add migration script here
CREATE TABLE subscripton_tokens (
    subscripton_token TEXT NOT NULL,
    subscriber_id uuid NOT NULL REFERENCEs subscriptions(id),
    PRIMARY KEY (subscripton_token)
);