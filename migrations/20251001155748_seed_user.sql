-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '5f28cbd8-7592-4255-bf43-ee1becae3750',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$bYcXYPu/VIfWlwOSf'
    'y9O+w$fHvjmwG3ujM+xvvdK7TJaoomRmtQvuw252Jn5vP5PlY'
);