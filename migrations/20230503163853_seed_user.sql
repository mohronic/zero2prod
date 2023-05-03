-- Add migration script here
INSERT INTO users(user_id, username, password_hash) 
VALUES (
    '7689b968-321e-421b-b065-9ca438fc6e91',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$JtOnPKkDq4T+QJqtwtFdRw$RyjMkdnDc/NSUxM3qGLaohjtNqjRjZ5KBvMPE9mkgRE'
);