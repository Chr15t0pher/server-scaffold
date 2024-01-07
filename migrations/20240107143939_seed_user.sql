-- Add migration script here
INSERT INTO users(user_id, username, password_hash)
VALUES(
  'e5e9c9e6-ce59-4556-982f-789d9b0aac10',
  'admin',
  '$argon2id$v=19$m=15000,t=2,p=1$Az3lhYrqwb9p5pckUQMoFA$j8K1V4ZtCIE+8+S869RWddSkRwnkpP8mmEmV8VIBoOk'
)