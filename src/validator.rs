//! Checksum and structural validators for PII recognizers.
//!
//! Each [`Validator`] variant implements a specific algorithm. Validators are
//! referenced by name from the TOML catalog via [`Validator::by_name`].

/// All supported checksum and structural validators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Validator {
    /// No validation; every regex match is accepted.
    None,
    /// Standard Luhn mod-10 (strips non-digits first).
    Luhn,
    /// Luhn + length 12..=19 digits (credit/debit cards).
    LuhnCard,
    /// IBAN mod-97.
    Iban,
    /// IPv4 structural (`std::net::Ipv4Addr` parse).
    Ipv4,
    /// IPv6 structural (`std::net::Ipv6Addr` parse).
    Ipv6,
    /// Verhoeff algorithm (Aadhaar).
    Verhoeff,
    /// UK NHS mod-11.
    NhsMod11,
    /// US SSN structural (area/group/serial rules).
    UsSsn,
    /// Spanish NIF letter check.
    EsNif,
    /// Spanish NIE letter check.
    EsNie,
    /// Italian fiscal code (Codice Fiscale).
    ItFiscal,
    /// Polish PESEL.
    PlPesel,
    /// Australian ABN.
    AuAbn,
    /// Australian ACN.
    AuAcn,
    /// Australian TFN.
    AuTfn,
    /// Singapore NRIC/FIN.
    SgNric,
    /// Indian GSTIN.
    Gstin,
    /// US ABA routing number.
    AbaRouting,
    /// Finnish HETU (henkilötunnus).
    FiHetu,
    /// Phone: strip non-digits, check 7..=15 digits.
    PhoneLoose,
    // ── Phase 4 validators ────────────────────────────────────────────────
    /// German Steuer-Identifikationsnummer (11-digit mod-11).
    DeTaxId,
    /// French NIR/INSEE (mod-97 on first 15 digits → 2-digit key).
    FrNir,
    /// Dutch BSN (11-test / Elfproef).
    NlBsn,
    /// Belgian National Register Number (mod-97).
    BeNrn,
    /// Brazilian CPF (mod-11 two check digits).
    BrCpf,
    /// Brazilian CNPJ (mod-11 two check digits).
    BrCnpj,
    /// Mexican CURP structural check digit.
    MxCurp,
    /// Norwegian fødselsnummer (two mod-11 checks).
    NoFnr,
    /// Portuguese NIF (mod-11).
    PtNif,
    /// Irish PPS number (mod-23 letter).
    IePps,
    /// Romanian CNP (weighted mod-11).
    RoCnp,
    /// Turkish TC Kimlik (TCKN) algorithm.
    TrTckn,
    /// Chinese RIC (18-digit; ISO 7064 MOD 11-2).
    CnRic,
    /// Japanese My Number (12-digit mod-11 check).
    JpMynumber,
    /// Chilean RUN/RUT (mod-11 with 'K' as 10).
    ClRun,
    /// Argentine CUIT/CUIL (mod-11).
    ArCuit,
    /// Russian INN (10-digit or 12-digit weighted checks).
    RuInn,
    /// Russian SNILS (mod-101 then mod-10).
    RuSnils,
}

impl Validator {
    /// Look up a validator by its catalog name. Returns `None` for unknown names.
    #[must_use]
    pub fn by_name(name: &str) -> Option<Self> {
        match name {
            "none" => Some(Self::None),
            "luhn" => Some(Self::Luhn),
            "luhn_card" => Some(Self::LuhnCard),
            "iban" => Some(Self::Iban),
            "ipv4" => Some(Self::Ipv4),
            "ipv6" => Some(Self::Ipv6),
            "verhoeff" => Some(Self::Verhoeff),
            "nhs_mod11" => Some(Self::NhsMod11),
            "us_ssn" => Some(Self::UsSsn),
            "es_nif" => Some(Self::EsNif),
            "es_nie" => Some(Self::EsNie),
            "it_fiscal" => Some(Self::ItFiscal),
            "pl_pesel" => Some(Self::PlPesel),
            "au_abn" => Some(Self::AuAbn),
            "au_acn" => Some(Self::AuAcn),
            "au_tfn" => Some(Self::AuTfn),
            "sg_nric" => Some(Self::SgNric),
            "gstin" => Some(Self::Gstin),
            "aba_routing" => Some(Self::AbaRouting),
            "fi_hetu" => Some(Self::FiHetu),
            "phone_loose" => Some(Self::PhoneLoose),
            "de_tax_id" => Some(Self::DeTaxId),
            "fr_nir" => Some(Self::FrNir),
            "nl_bsn" => Some(Self::NlBsn),
            "be_nrn" => Some(Self::BeNrn),
            "br_cpf" => Some(Self::BrCpf),
            "br_cnpj" => Some(Self::BrCnpj),
            "mx_curp" => Some(Self::MxCurp),
            "no_fnr" => Some(Self::NoFnr),
            "pt_nif" => Some(Self::PtNif),
            "ie_pps" => Some(Self::IePps),
            "ro_cnp" => Some(Self::RoCnp),
            "tr_tckn" => Some(Self::TrTckn),
            "cn_ric" => Some(Self::CnRic),
            "jp_mynumber" => Some(Self::JpMynumber),
            "cl_run" => Some(Self::ClRun),
            "ar_cuit" => Some(Self::ArCuit),
            "ru_inn" => Some(Self::RuInn),
            "ru_snils" => Some(Self::RuSnils),
            _ => None,
        }
    }

    /// Returns `true` if the matched string passes this validator.
    #[must_use]
    pub fn validate(self, m: &str) -> bool {
        match self {
            Self::None => true,
            Self::Luhn => luhn(m),
            Self::LuhnCard => luhn_card(m),
            Self::Iban => iban(m),
            Self::Ipv4 => m.trim().parse::<std::net::Ipv4Addr>().is_ok(),
            Self::Ipv6 => m.trim().parse::<std::net::Ipv6Addr>().is_ok(),
            Self::Verhoeff => verhoeff(m),
            Self::NhsMod11 => nhs_mod11(m),
            Self::UsSsn => us_ssn(m),
            Self::EsNif => es_nif(m),
            Self::EsNie => es_nie(m),
            Self::ItFiscal => it_fiscal(m),
            Self::PlPesel => pl_pesel(m),
            Self::AuAbn => au_abn(m),
            Self::AuAcn => au_acn(m),
            Self::AuTfn => au_tfn(m),
            Self::SgNric => sg_nric(m),
            Self::Gstin => gstin(m),
            Self::AbaRouting => aba_routing(m),
            Self::FiHetu => fi_hetu(m),
            Self::PhoneLoose => phone_loose(m),
            Self::DeTaxId => de_tax_id(m),
            Self::FrNir => fr_nir(m),
            Self::NlBsn => nl_bsn(m),
            Self::BeNrn => be_nrn(m),
            Self::BrCpf => br_cpf(m),
            Self::BrCnpj => br_cnpj(m),
            Self::MxCurp => mx_curp(m),
            Self::NoFnr => no_fnr(m),
            Self::PtNif => pt_nif(m),
            Self::IePps => ie_pps(m),
            Self::RoCnp => ro_cnp(m),
            Self::TrTckn => tr_tckn(m),
            Self::CnRic => cn_ric(m),
            Self::JpMynumber => jp_mynumber(m),
            Self::ClRun => cl_run(m),
            Self::ArCuit => ar_cuit(m),
            Self::RuInn => ru_inn(m),
            Self::RuSnils => ru_snils(m),
        }
    }

    /// Whether this validator applies any checksum logic (vs pure structural/regex).
    #[must_use]
    pub fn has_checksum(self) -> bool {
        !matches!(self, Self::None | Self::Ipv4 | Self::Ipv6 | Self::PhoneLoose | Self::UsSsn)
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Extract ASCII decimal digits from `m`.
fn digits(m: &str) -> Vec<u8> {
    m.bytes().filter(|b| b.is_ascii_digit()).map(|b| b - b'0').collect()
}

// ── Core validators ───────────────────────────────────────────────────────────

/// Standard Luhn mod-10 (no length constraint).
fn luhn(m: &str) -> bool {
    let d = digits(m);
    if d.is_empty() {
        return false;
    }
    let sum: u32 = d
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| {
            if i % 2 == 1 {
                let v = u32::from(x) * 2;
                if v > 9 {
                    v - 9
                } else {
                    v
                }
            } else {
                u32::from(x)
            }
        })
        .sum();
    sum % 10 == 0
}

/// Luhn + length 12..=19 (credit/debit cards).
fn luhn_card(m: &str) -> bool {
    let d = digits(m);
    let n = d.len();
    if !(12..=19).contains(&n) {
        return false;
    }
    let sum: u32 = d
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| {
            if i % 2 == 1 {
                let v = u32::from(x) * 2;
                if v > 9 {
                    v - 9
                } else {
                    v
                }
            } else {
                u32::from(x)
            }
        })
        .sum();
    sum % 10 == 0
}

/// IBAN mod-97 validation.
fn iban(m: &str) -> bool {
    let s: String =
        m.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_uppercase();
    let len = s.len();
    if !(15..=34).contains(&len) {
        return false;
    }
    let mut chars = s.chars();
    if !chars.by_ref().take(2).all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    if !s[2..4].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // Rearrange: body + first-4
    let rearranged: String = s[4..].chars().chain(s[..4].chars()).collect();
    // Convert to numeric string
    let numeric: String = rearranged
        .chars()
        .flat_map(|c| {
            if c.is_ascii_alphabetic() {
                let v = u32::from(c) - u32::from('A') + 10;
                v.to_string().into_bytes()
            } else {
                vec![c as u8]
            }
        })
        .map(char::from)
        .collect();
    // Compute mod 97 digit-by-digit
    let mut rem: u32 = 0;
    for c in numeric.chars() {
        let d = c as u32 - '0' as u32;
        rem = (rem * 10 + d) % 97;
    }
    rem == 1
}

/// Verhoeff algorithm (used for Aadhaar).
fn verhoeff(m: &str) -> bool {
    #[rustfmt::skip]
    const D: [[u8; 10]; 10] = [
        [0,1,2,3,4,5,6,7,8,9],
        [1,2,3,4,0,6,7,8,9,5],
        [2,3,4,0,1,7,8,9,5,6],
        [3,4,0,1,2,8,9,5,6,7],
        [4,0,1,2,3,9,5,6,7,8],
        [5,9,8,7,6,0,4,3,2,1],
        [6,5,9,8,7,1,0,4,3,2],
        [7,6,5,9,8,2,1,0,4,3],
        [8,7,6,5,9,3,2,1,0,4],
        [9,8,7,6,5,4,3,2,1,0],
    ];
    #[rustfmt::skip]
    const P: [[u8; 10]; 8] = [
        [0,1,2,3,4,5,6,7,8,9],
        [1,5,7,6,2,8,3,0,9,4],
        [5,8,0,3,7,9,6,1,4,2],
        [8,9,1,6,0,4,3,5,2,7],
        [9,4,5,3,1,2,6,8,7,0],
        [4,2,8,6,5,7,3,9,0,1],
        [2,7,9,3,8,0,6,4,1,5],
        [7,0,4,6,9,1,3,2,5,8],
    ];
    let d = digits(m);
    if d.is_empty() {
        return false;
    }
    let mut c: u8 = 0;
    for (i, &digit) in d.iter().rev().enumerate() {
        c = D[usize::from(c)][usize::from(P[i % 8][usize::from(digit)])];
    }
    c == 0
}

/// UK NHS mod-11. 10 digits; weights [10..2] on first 9; check = 11-(sum%11); 11→0; 10=invalid.
fn nhs_mod11(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 10 {
        return false;
    }
    const WEIGHTS: [u32; 9] = [10, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = d[..9].iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    let check = 11 - (sum % 11);
    if check == 10 {
        return false;
    }
    let check = if check == 11 { 0 } else { check };
    check == u32::from(d[9])
}

/// US SSN structural rules (area/group/serial).
fn us_ssn(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 9 {
        return false;
    }
    let area = u32::from(d[0]) * 100 + u32::from(d[1]) * 10 + u32::from(d[2]);
    let group = u32::from(d[3]) * 10 + u32::from(d[4]);
    let serial =
        u32::from(d[5]) * 1000 + u32::from(d[6]) * 100 + u32::from(d[7]) * 10 + u32::from(d[8]);
    area != 0 && area != 666 && area < 900 && group != 0 && serial != 0
}

/// Spanish NIF: 8 digits + control letter.
fn es_nif(m: &str) -> bool {
    const TABLE: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";
    let s: String =
        m.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_uppercase();
    if s.len() != 9 {
        return false;
    }
    let (num_str, letter_str) = s.split_at(8);
    if !num_str.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let Ok(n) = num_str.parse::<u32>() else { return false };
    let letter = letter_str.chars().next().unwrap_or('\0');
    letter == TABLE[(n % 23) as usize] as char
}

/// Spanish NIE: X/Y/Z prefix + 7 digits + control letter.
fn es_nie(m: &str) -> bool {
    let s: String =
        m.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_uppercase();
    if s.len() != 9 {
        return false;
    }
    let prefix = match s.chars().next().unwrap_or('\0') {
        'X' => '0',
        'Y' => '1',
        'Z' => '2',
        _ => return false,
    };
    let modified = format!("{}{}", prefix, &s[1..]);
    es_nif(&modified)
}

/// Italian Codice Fiscale (16-char alphanumeric).
fn it_fiscal(m: &str) -> bool {
    let s: String =
        m.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_uppercase();
    if s.len() != 16 {
        return false;
    }
    let chars: Vec<char> = s.chars().collect();
    // Odd-position values (0-indexed even: positions 1,3,5,... in 1-based counting)
    let odd_val = |c: char| -> u32 {
        match c {
            '0' | 'A' => 1,
            '1' | 'B' => 0,
            '2' | 'C' => 5,
            '3' | 'D' => 7,
            '4' | 'E' => 9,
            '5' | 'F' => 13,
            '6' | 'G' => 15,
            '7' | 'H' => 17,
            '8' | 'I' => 19,
            '9' | 'J' => 21,
            'K' => 2,
            'L' => 4,
            'M' => 18,
            'N' => 20,
            'O' => 11,
            'P' => 3,
            'Q' => 6,
            'R' => 8,
            'S' => 12,
            'T' => 14,
            'U' => 16,
            'V' => 10,
            'W' => 22,
            'X' => 25,
            'Y' => 24,
            'Z' => 23,
            _ => 0,
        }
    };
    // Even-position values (0-indexed odd: positions 2,4,6,... in 1-based counting)
    let even_val = |c: char| -> u32 {
        if c.is_ascii_digit() {
            u32::from(c) - u32::from('0')
        } else if c.is_ascii_uppercase() {
            u32::from(c) - u32::from('A')
        } else {
            0
        }
    };
    let sum: u32 = chars[..15]
        .iter()
        .enumerate()
        .map(|(i, &c)| if i % 2 == 0 { odd_val(c) } else { even_val(c) })
        .sum();
    let expected = (b'A' + (sum % 26) as u8) as char;
    chars[15] == expected
}

/// Polish PESEL (11-digit, weighted mod-10).
fn pl_pesel(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    const WEIGHTS: [u32; 10] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];
    let sum: u32 = d[..10].iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    let check = (10 - (sum % 10)) % 10;
    check == u32::from(d[10])
}

/// Australian ABN (11 digits).
fn au_abn(m: &str) -> bool {
    let mut d = digits(m);
    if d.len() != 11 {
        return false;
    }
    if d[0] == 0 {
        return false;
    }
    d[0] -= 1;
    const WEIGHTS: [u32; 11] = [10, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let sum: u32 = d.iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    sum % 89 == 0
}

/// Australian ACN (9 digits).
fn au_acn(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 9 {
        return false;
    }
    const WEIGHTS: [u32; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
    let sum: u32 = d[..8].iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    let check = (10 - (sum % 10)) % 10;
    check == u32::from(d[8])
}

/// Australian TFN (8 or 9 digits).
fn au_tfn(m: &str) -> bool {
    let d = digits(m);
    let d: Vec<u8> = match d.len() {
        8 => {
            let mut v = vec![0u8];
            v.extend_from_slice(&d);
            v
        },
        9 => d,
        _ => return false,
    };
    const WEIGHTS: [u32; 9] = [1, 4, 3, 7, 5, 8, 6, 9, 10];
    let sum: u32 = d.iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    sum % 11 == 0
}

/// Singapore NRIC/FIN checksum.
fn sg_nric(m: &str) -> bool {
    let s = m.trim().to_ascii_uppercase();
    if s.len() != 9 {
        return false;
    }
    let b = s.as_bytes();
    let prefix = b[0] as char;
    let suffix = b[8] as char;
    if !matches!(prefix, 'S' | 'T' | 'F' | 'G' | 'M') {
        return false;
    }
    if !s[1..8].bytes().all(|c| c.is_ascii_digit()) {
        return false;
    }
    const WEIGHTS: [u32; 7] = [2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = s[1..8].bytes().zip(WEIGHTS.iter()).map(|(c, &w)| u32::from(c - b'0') * w).sum();
    let (offset, table): (u32, &[u8]) = match prefix {
        'S' => (0, b"JZIHGFEDCBA"),
        'T' => (4, b"JZIHGFEDCBA"),
        'F' => (0, b"XWUTRQPNMLK"),
        'G' => (4, b"XWUTRQPNMLK"),
        'M' => (3, b"XWUTRQPNMLK"),
        _ => return false,
    };
    let idx = ((sum + offset) % 11) as usize;
    idx < table.len() && suffix == table[idx] as char
}

/// Indian GSTIN check digit (base-36 Luhn-style mod-36).
fn gstin(m: &str) -> bool {
    let s = m.trim().to_ascii_uppercase();
    if s.len() != 15 {
        return false;
    }
    if !s.bytes().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }
    let b = s.as_bytes();
    let char_val = |c: u8| -> u32 {
        if c.is_ascii_digit() {
            u32::from(c - b'0')
        } else {
            u32::from(c - b'A') + 10
        }
    };
    let mut total: u32 = 0;
    for (i, &c) in b[..14].iter().enumerate() {
        let val = char_val(c);
        let factor = if i % 2 == 0 { 1u32 } else { 2 };
        let product = val * factor;
        total += product / 36 + product % 36;
    }
    let check_val = (36 - (total % 36)) % 36;
    let expected =
        if check_val < 10 { b'0' + check_val as u8 } else { b'A' + (check_val - 10) as u8 };
    b[14] == expected
}

/// US ABA routing number (9 digits, weighted mod-10).
fn aba_routing(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 9 {
        return false;
    }
    const WEIGHTS: [u32; 9] = [3, 7, 1, 3, 7, 1, 3, 7, 1];
    let sum: u32 = d.iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    sum % 10 == 0
}

/// Finnish HETU (henkilötunnus) check character.
fn fi_hetu(m: &str) -> bool {
    // Format: ddmmyyXnnnC where X is -, +, or A; nnn is 3 digits; C is check char
    let m = m.trim();
    if m.len() != 11 {
        return false;
    }
    let b = m.as_bytes();
    // Validate date and separator chars
    if !b[..6].iter().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if !matches!(b[6], b'-' | b'+' | b'A') {
        return false;
    }
    if !b[7..10].iter().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // Compute: num = ddmmyyNNN as a 9-digit integer
    let num_str = format!(
        "{}{}",
        std::str::from_utf8(&b[..6]).unwrap_or(""),
        std::str::from_utf8(&b[7..10]).unwrap_or("")
    );
    let Ok(num) = num_str.parse::<u64>() else { return false };
    const TABLE: &[u8] = b"0123456789ABCDEFHJKLMNPRSTUVWXY";
    let expected = TABLE[(num % 31) as usize] as char;
    b[10].to_ascii_uppercase() as char == expected
}

/// Phone loose: strip non-digits; accept 7..=15 digits.
#[cfg(not(feature = "phone"))]
fn phone_loose(m: &str) -> bool {
    let count = m.bytes().filter(|b| b.is_ascii_digit()).count();
    (7..=15).contains(&count)
}

/// Phone strict: delegate to the `phonenumber` crate when `phone` feature is enabled.
#[cfg(feature = "phone")]
fn phone_loose(m: &str) -> bool {
    phonenumber::parse(None, m).map(|p| phonenumber::is_valid(&p)).unwrap_or(false)
}

// ── Phase 4 validators ─────────────────────────────────────────────────────────

/// German Steuer-IdNr (11 digits).
/// Algorithm: first digit 1-9; digits 1-10 weighted by their occurrence count;
/// standard mod-11 variant.
fn de_tax_id(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    if d[0] == 0 {
        return false;
    }
    // No digit 0-9 may appear more than 3× in positions 1-10; at most one digit appears ≥3×.
    // Simplified: verify check digit via the published mod-11 variant.
    // product method: start with product=10 (instead of row), iterate positions 1-10
    let mut product = 10u32;
    for &x in &d[..10] {
        let sum = (product + u32::from(x)) % 10;
        let sum = if sum == 0 { 10 } else { sum };
        product = (sum * 2) % 11;
    }
    let check = (11 - product) % 10;
    check == u32::from(d[10])
}

/// French NIR (INSEE / social security): 15-digit body + 2-digit key.
/// key = 97 - (body mod 97).
fn fr_nir(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 15 {
        return false;
    }
    // body = first 13 digits as integer (mod 97)
    let body_str: String = d[..13].iter().map(|&x| (b'0' + x) as char).collect();
    let Ok(body) = body_str.parse::<u64>() else { return false };
    let key: u32 = (d[13] as u32) * 10 + d[14] as u32;
    key == (97 - (body % 97) as u32)
}

/// Dutch BSN (Elfproef / 11-test).
fn nl_bsn(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 9 {
        return false;
    }
    // weights: 9,8,7,6,5,4,3,2,-1
    let sum: i32 =
        d[..8].iter().enumerate().map(|(i, &x)| i32::from(x) * (9 - i as i32)).sum::<i32>()
            - i32::from(d[8]);
    sum != 0 && sum % 11 == 0
}

/// Belgian National Register Number (11 digits, mod-97).
fn be_nrn(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    // first 9 digits as number; check = 97 - (n mod 97); or for born ≥ 2000: n += 2_000_000_000
    let body_str: String = d[..9].iter().map(|&x| (b'0' + x) as char).collect();
    let Ok(n) = body_str.parse::<u64>() else { return false };
    let check: u32 = (d[9] as u32) * 10 + d[10] as u32;
    let try_check = |base: u64| -> bool { (97 - (base % 97) as u32) == check };
    try_check(n) || try_check(n + 2_000_000_000)
}

/// Brazilian CPF (11 digits, two mod-11 check digits).
fn br_cpf(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    // reject all-same digits
    if d.iter().all(|&x| x == d[0]) {
        return false;
    }
    let check1 = |d: &[u8]| -> u8 {
        let sum: u32 =
            d[..9].iter().enumerate().map(|(i, &x)| u32::from(x) * (10 - i as u32)).sum();
        let rem = sum % 11;
        if rem < 2 {
            0
        } else {
            (11 - rem) as u8
        }
    };
    let check2 = |d: &[u8]| -> u8 {
        let sum: u32 =
            d[..10].iter().enumerate().map(|(i, &x)| u32::from(x) * (11 - i as u32)).sum();
        let rem = sum % 11;
        if rem < 2 {
            0
        } else {
            (11 - rem) as u8
        }
    };
    d[9] == check1(&d) && d[10] == check2(&d)
}

/// Brazilian CNPJ (14 digits, two mod-11 check digits).
fn br_cnpj(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 14 {
        return false;
    }
    if d.iter().all(|&x| x == d[0]) {
        return false;
    }
    let w1: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let w2: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let check = |slice: &[u8], weights: &[u32]| -> u8 {
        let sum: u32 = slice.iter().zip(weights.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
        let rem = sum % 11;
        if rem < 2 {
            0
        } else {
            (11 - rem) as u8
        }
    };
    d[12] == check(&d[..12], &w1) && d[13] == check(&d[..13], &w2)
}

/// Mexican CURP check digit (alphanumeric; 18 chars).
fn mx_curp(m: &str) -> bool {
    let s = m.trim().to_ascii_uppercase();
    if s.len() != 18 {
        return false;
    }
    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let val_of = |c: u8| -> Option<u32> { CHARSET.iter().position(|&x| x == c).map(|p| p as u32) };
    let b = s.as_bytes();
    let mut sum: u32 = 0;
    for (i, &c) in b[..17].iter().enumerate() {
        let Some(v) = val_of(c) else { return false };
        sum += v * (18 - i as u32);
    }
    let check = (10 - (sum % 10)) % 10;
    let Some(last) = val_of(b[17]) else { return false };
    last == check
}

/// Norwegian fødselsnummer (11 digits, two mod-11 checks).
fn no_fnr(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    const W1: [u32; 9] = [3, 7, 6, 1, 8, 9, 4, 5, 2];
    const W2: [u32; 10] = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let check = |slice: &[u8], weights: &[u32], check_pos: usize| -> bool {
        let sum: u32 = slice[..weights.len()]
            .iter()
            .zip(weights.iter())
            .map(|(&x, &w)| u32::from(x) * w)
            .sum();
        let rem = sum % 11;
        let c = if rem == 0 { 0 } else { 11 - rem };
        c < 10 && c == u32::from(slice[check_pos])
    };
    check(&d, &W1, 9) && check(&d, &W2, 10)
}

/// Portuguese NIF (9 digits, mod-11).
fn pt_nif(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 9 {
        return false;
    }
    let sum: u32 = d[..8].iter().enumerate().map(|(i, &x)| u32::from(x) * (9 - i as u32)).sum();
    let rem = sum % 11;
    let check = if rem < 2 { 0 } else { 11 - rem };
    check == u32::from(d[8])
}

/// Irish PPS number (7 digits + 1 or 2 letters; mod-23 check).
fn ie_pps(m: &str) -> bool {
    let s = m.trim().to_ascii_uppercase();
    // Format: 7 digits + 1 letter (old) or 7 digits + 2 letters (new, second = W)
    let b = s.as_bytes();
    if b.len() != 8 && b.len() != 9 {
        return false;
    }
    if !b[..7].iter().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if !b[7..].iter().all(|c| c.is_ascii_uppercase()) {
        return false;
    }
    const WEIGHTS: [u32; 8] = [8, 7, 6, 5, 4, 3, 2, 9];
    let sum: u32 =
        b[..7].iter().zip(WEIGHTS.iter()).map(|(&c, &w)| u32::from(c - b'0') * w).sum::<u32>()
            + if b.len() == 9 {
                // second letter contributes at weight 9 only for 9-char forms
                u32::from(b[8] - b'A' + 1) * 9
            } else {
                0
            };
    let check = (sum % 23) as u8;
    b[7] == check + b'A'
}

/// Romanian CNP (13 digits, weighted mod-11).
fn ro_cnp(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 13 {
        return false;
    }
    const WEIGHTS: [u32; 12] = [2, 7, 9, 1, 4, 6, 3, 5, 8, 2, 7, 9];
    let sum: u32 = d[..12].iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    let rem = sum % 11;
    let check = if rem == 10 { 1 } else { rem };
    check == u32::from(d[12])
}

/// Turkish TCKN (11 digits).
fn tr_tckn(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    if d[0] == 0 {
        return false;
    }
    // d[9] = (7 * sum(d[0..7:2]) - sum(d[1..7:2])) % 10
    let odd_sum: u32 = d[0..=8].iter().step_by(2).map(|&x| u32::from(x)).sum(); // d0+d2+d4+d6+d8
    let even_sum: u32 = d[1..=8].iter().step_by(2).map(|&x| u32::from(x)).sum(); // d1+d3+d5+d7
                                                                                 // digit 10 check
    let check10 = (odd_sum * 7).wrapping_sub(even_sum) % 10;
    if check10 != u32::from(d[9]) {
        return false;
    }
    // digit 11 check: sum of all first 10 digits mod 10
    let total: u32 = d[..10].iter().map(|&x| u32::from(x)).sum();
    total % 10 == u32::from(d[10])
}

/// Chinese RIC (18-char, ISO 7064 MOD 11-2).
fn cn_ric(m: &str) -> bool {
    let s = m.trim().to_ascii_uppercase();
    if s.len() != 18 {
        return false;
    }
    let b = s.as_bytes();
    if !b[..17].iter().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // Check char: digit or 'X'
    if !matches!(b[17], b'0'..=b'9' | b'X') {
        return false;
    }
    const WEIGHTS: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    const CHECKS: &[u8] = b"10X98765432";
    let sum: u32 = b[..17].iter().zip(WEIGHTS.iter()).map(|(&c, &w)| u32::from(c - b'0') * w).sum();
    let expected = CHECKS[(sum % 11) as usize];
    b[17] == expected
}

/// Japanese My Number (12 digits, mod-11 check).
fn jp_mynumber(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 12 {
        return false;
    }
    const WEIGHTS: [u32; 11] = [6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = d[..11].iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    let rem = sum % 11;
    let check = if rem <= 1 { 0 } else { 11 - rem };
    check == u32::from(d[11])
}

/// Chilean RUN/RUT check digit (mod-11, 'K' = 10).
fn cl_run(m: &str) -> bool {
    let s = m.trim().to_ascii_uppercase();
    // Format: digits with optional dash before check, check is 0-9 or K
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let check_char = *bytes.last().unwrap();
    if !matches!(check_char, b'0'..=b'9' | b'K') {
        return false;
    }
    // Body: all digits before the check char (ignore dashes and dots)
    let body: Vec<u8> =
        bytes[..bytes.len() - 1].iter().filter(|c| c.is_ascii_digit()).map(|&c| c - b'0').collect();
    if body.is_empty() {
        return false;
    }
    const CYCLE: [u32; 6] = [2, 3, 4, 5, 6, 7];
    let sum: u32 = body.iter().rev().enumerate().map(|(i, &x)| u32::from(x) * CYCLE[i % 6]).sum();
    let check = 11 - (sum % 11);
    let expected = match check {
        11 => b'0',
        10 => b'K',
        c => b'0' + c as u8,
    };
    check_char == expected
}

/// Argentine CUIT/CUIL (11 digits, mod-11).
fn ar_cuit(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    const WEIGHTS: [u32; 10] = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = d[..10].iter().zip(WEIGHTS.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
    let rem = sum % 11;
    let check = if rem == 0 { 0 } else { 11 - rem };
    check < 10 && check == u32::from(d[10])
}

/// Russian INN (10-digit or 12-digit weighted checks).
fn ru_inn(m: &str) -> bool {
    let d = digits(m);
    match d.len() {
        10 => {
            const W: [u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
            let sum: u32 = d[..9].iter().zip(W.iter()).map(|(&x, &w)| u32::from(x) * w).sum();
            (sum % 11) % 10 == u32::from(d[9])
        },
        12 => {
            const W1: [u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
            const W2: [u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
            let c1 = (d[..10].iter().zip(W1.iter()).map(|(&x, &w)| u32::from(x) * w).sum::<u32>()
                % 11)
                % 10;
            let c2 = (d[..11].iter().zip(W2.iter()).map(|(&x, &w)| u32::from(x) * w).sum::<u32>()
                % 11)
                % 10;
            c1 == u32::from(d[10]) && c2 == u32::from(d[11])
        },
        _ => false,
    }
}

/// Russian SNILS (11 digits; checksum: sum of first 9 weighted by position; mod 101 then mod 10).
fn ru_snils(m: &str) -> bool {
    let d = digits(m);
    if d.len() != 11 {
        return false;
    }
    let sum: u32 = d[..9].iter().enumerate().map(|(i, &x)| u32::from(x) * (9 - i as u32)).sum();
    let check = match sum % 101 {
        100 | 101 => 0,
        n => n,
    };
    check == u32::from(d[9]) * 10 + u32::from(d[10])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luhn_known_vectors() {
        assert!(luhn("4111111111111111"), "Visa test card must pass Luhn");
        assert!(!luhn("4111111111111112"), "Flipped last digit must fail");
        assert!(luhn("79927398713"), "Known-valid short Luhn");
        assert!(!luhn("79927398710"), "Known-invalid short Luhn");
    }

    #[test]
    fn luhn_card_length_gate() {
        // Valid 15-digit Amex
        assert!(luhn_card("378282246310005"));
        // Valid 16-digit Visa
        assert!(luhn_card("4111111111111111"));
        // 11 digits — too short
        assert!(!luhn_card("37828224631"));
        // Luhn-valid but only 11 digits
        assert!(!luhn_card("79927398713"));
    }

    #[test]
    fn iban_known_vectors() {
        // Source: https://www.iban.com/structure
        assert!(iban("DE89370400440532013000"));
        assert!(iban("GB29NWBK60161331926819"));
        assert!(iban("FR7630006000011234567890189"));
        assert!(!iban("DE89370400440532013001"), "last digit off");
        assert!(!iban("NOTANIBAN"));
    }

    #[test]
    fn verhoeff_known_vectors() {
        // Verhoeff check on itself must pass
        // "2363" is a known-valid Verhoeff test vector
        assert!(verhoeff("2363"));
        assert!(!verhoeff("2364"));
    }

    #[test]
    fn nhs_known_vectors() {
        // Source: NHS number validation spec
        assert!(nhs_mod11("9434765919"));
        assert!(!nhs_mod11("9434765910"));
        // 11th-step produces c=10 → invalid structure
        assert!(!nhs_mod11("1234567890"));
    }

    #[test]
    fn us_ssn_structural() {
        assert!(us_ssn("219099999"), "valid SSN digits");
        assert!(!us_ssn("000099999"), "area 000");
        assert!(!us_ssn("666099999"), "area 666");
        assert!(!us_ssn("900099999"), "area >= 900");
        assert!(!us_ssn("219009999"), "group 00");
        assert!(!us_ssn("219990000"), "serial 0000");
    }

    #[test]
    fn es_nif_nie() {
        // Source: Spanish tax agency algorithm
        assert!(es_nif("12345678Z"));
        assert!(!es_nif("12345678A"));
        assert!(es_nie("X1234567L"));
    }

    #[test]
    fn au_abn_known() {
        // Source: https://abr.business.gov.au/Help/AbnFormat
        assert!(au_abn("51824753556"));
        assert!(!au_abn("51824753557"));
    }

    #[test]
    fn aba_routing_known() {
        // Source: https://en.wikipedia.org/wiki/ABA_routing_transit_number
        assert!(aba_routing("322271627"));
        assert!(!aba_routing("122271627"));
    }

    #[test]
    fn pl_pesel_check() {
        // PESEL check — known vector
        assert!(pl_pesel("44051401359"));
        assert!(!pl_pesel("44051401350"));
    }

    #[test]
    fn gstin_known() {
        // Source: verified via GST portal algorithm
        assert!(gstin("27AAPFU0939F1ZV"));
        assert!(!gstin("27AAPFU0939F1ZW"));
    }

    #[test]
    fn cn_ric_known() {
        // Source: ISO 7064 MOD 11-2
        assert!(cn_ric("11010519491231002X"));
        assert!(!cn_ric("11010519491231002Y"));
    }

    #[test]
    fn tr_tckn_known() {
        // Source: Turkish TCKN algorithm spec
        assert!(tr_tckn("10000000146"));
        assert!(!tr_tckn("10000000147"));
    }
}
