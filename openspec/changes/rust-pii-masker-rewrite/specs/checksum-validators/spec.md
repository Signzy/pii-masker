## ADDED Requirements

### Requirement: Validator registry
Every validator MUST be addressable by a lowercase snake_case string name via
`Validator::by_name(&str) -> Option<Validator>`. An unknown name in a catalog entry MUST produce
`Error::Catalog` at catalog load time, not at runtime masking.

#### Scenario: Known validator resolves
- **WHEN** `Validator::by_name("luhn")` is called
- **THEN** it returns `Some(Validator::Luhn)`

#### Scenario: Unknown validator errors at catalog load
- **WHEN** a TOML catalog entry contains `validator = "unknown_xyz"`
- **THEN** `catalog::recognizers()` returns `Err(Error::Catalog(...))`

### Requirement: Luhn (credit cards, CA SIN, ZA ID, SE personnummer)
- **WHEN** `Validator::Luhn.validate(s)` is called
- **THEN** it strips non-digit characters and applies standard Luhn mod-10
- `4111111111111111` is valid
- `4111111111111112` is invalid

### Requirement: LuhnCard (strict card length)
- **WHEN** `Validator::LuhnCard.validate(s)` is called
- **THEN** it strips separators, checks `len in 12..=19`, then applies Luhn
- `4111 1111 1111 1111` is valid (Visa, 16 digits)
- `4111111111111112` is invalid (fails Luhn)

### Requirement: IBAN (mod-97)
- Move first 4 chars to end, map letters to 10-35, compute running mod-97
- `DE89370400440532013000` is valid
- `DE89370400440532013001` is invalid
- `GB29NWBK60161331926819` is valid

### Requirement: Verhoeff (Aadhaar)
- Standard D/P/INV table algorithm; right-to-left; valid iff accumulator == 0
- At least 3 published valid Aadhaar samples must pass
- Any digit transposition of a valid number must fail

### Requirement: NHS mod-11
- Weights [10,9,8,7,6,5,4,3,2] on first 9 digits; c = 11 - (sum % 11); 11 becomes 0; 10 is invalid
- `9434765919` is valid
- `9434765918` is invalid

### Requirement: US SSN structural
- Strip hyphens; area not 000/666 and < 900; group not 00; serial not 0000
- `219-09-9999` is valid
- `000-09-9999` is invalid (area 000)
- `219-00-9999` is invalid (group 00)

### Requirement: ABA routing (US)
- 9 digits; weights [3,7,1,3,7,1,3,7,1]; sum % 10 == 0
- `322271627` is valid
- `122271627` is invalid

### Requirement: ES NIF / NIE
- NIF: 8 digits + letter == TRWAGMYFPDXBNJZSQVHLCKE[n % 23]
- NIE: first X/Y/Z -> 0/1/2, prepend to 7 digits, then NIF letter check
- `12345678Z` is a valid NIF
- `X1234567L` is a valid NIE

### Requirement: IT fiscal code
- 16 chars; odd/even position value tables; sum % 26 maps to control letter A..Z
- At least 3 published valid samples must pass

### Requirement: PL PESEL
- 11 digits; weights [1,3,7,9,1,3,7,9,1,3]; c = (10-(sum%10))%10; c == d11
- At least 3 published valid samples must pass

### Requirement: AU ABN / ACN / TFN
- ABN: subtract 1 from first digit; weights [10,1,3,5,7,9,11,13,15,17,19]; sum % 89 == 0
- ACN: weights [8,7,6,5,4,3,2,1] on first 8; c=(10-(sum%10))%10; c==d9
- TFN: 8-9 digits; weights [1,4,3,7,5,8,6,9,10]; sum % 11 == 0
- `51824753556` is a valid ABN

### Requirement: SG NRIC
- Prefix S/T/F/G/M selects weight table and checksum letter table
- Weights [2,7,6,5,4,3,2] on 7 digits; offset added for T/G/M prefix
- At least 3 published valid samples must pass

### Requirement: GSTIN (India)
- 15 chars; base-36 Luhn-style mod-36 on first 14 chars yields char 15
- At least 3 published valid samples must pass

### Requirement: FI HETU
- ddmmyy + sign (-, +, A) + 3 digits; control = "0123456789ABCDEFHJKLMNPRSTUVWXY"[num % 31]
  where num = the 9-digit ddmmyyNNN integer
- At least 3 published valid samples must pass

### Requirement: PhoneLoose
- Strip non-digits; valid if 7..=15 digits remain
- With `phone` feature: delegates to phonenumber::parse(None, m).map(|p| p.is_valid())

### Requirement: IPv4 / IPv6 structural
- Delegate to m.trim().parse::<std::net::Ipv4Addr>() / Ipv6Addr
- `192.168.1.1` is valid IPv4
- `999.0.0.1` is invalid IPv4
- `2001:db8::1` is valid IPv6
