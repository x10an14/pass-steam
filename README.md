# pass-steam

A small utility meant to allow generation of Steam Guard's 2FA TOTP codes.
Run `pass-steam -/h--help` for more info.

### Dev usage example
`pass show services/steampowered.com/x10an14@gmail.com/2fa/maFile.json | jq -r ".shared_secret" | cargo run`

### Dev build
`nix build`
