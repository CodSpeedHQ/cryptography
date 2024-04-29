// This file is dual licensed under the terms of the Apache License, Version
// 2.0, and the BSD License. See the LICENSE file in the root of this repository
// for complete details.

use asn1::Asn1DefinedByWritable;

use crate::oid;

#[derive(asn1::Asn1Read, asn1::Asn1Write, PartialEq, Hash, Clone, Eq, Debug)]
pub struct AlgorithmIdentifier<'a> {
    pub oid: asn1::DefinedByMarker<asn1::ObjectIdentifier>,
    #[defined_by(oid)]
    pub params: AlgorithmParameters<'a>,
}

impl AlgorithmIdentifier<'_> {
    pub fn oid(&self) -> &asn1::ObjectIdentifier {
        self.params.item()
    }
}

#[derive(asn1::Asn1DefinedByRead, asn1::Asn1DefinedByWrite, PartialEq, Eq, Hash, Clone, Debug)]
pub enum AlgorithmParameters<'a> {
    #[defined_by(oid::SHA1_OID)]
    Sha1(Option<asn1::Null>),
    #[defined_by(oid::SHA224_OID)]
    Sha224(Option<asn1::Null>),
    #[defined_by(oid::SHA256_OID)]
    Sha256(Option<asn1::Null>),
    #[defined_by(oid::SHA384_OID)]
    Sha384(Option<asn1::Null>),
    #[defined_by(oid::SHA512_OID)]
    Sha512(Option<asn1::Null>),
    #[defined_by(oid::SHA3_224_OID)]
    Sha3_224(Option<asn1::Null>),
    #[defined_by(oid::SHA3_256_OID)]
    Sha3_256(Option<asn1::Null>),
    #[defined_by(oid::SHA3_384_OID)]
    Sha3_384(Option<asn1::Null>),
    #[defined_by(oid::SHA3_512_OID)]
    Sha3_512(Option<asn1::Null>),

    #[defined_by(oid::ED25519_OID)]
    Ed25519,
    #[defined_by(oid::ED448_OID)]
    Ed448,

    #[defined_by(oid::X25519_OID)]
    X25519,
    #[defined_by(oid::X448_OID)]
    X448,

    // These encodings are only used in SPKI AlgorithmIdentifiers.
    #[defined_by(oid::EC_OID)]
    Ec(EcParameters<'a>),

    #[defined_by(oid::RSA_OID)]
    Rsa(Option<asn1::Null>),

    // Used only in PKCS#7 AlgorithmIdentifiers
    // https://datatracker.ietf.org/doc/html/rfc3565#section-4.1
    //
    // From RFC 3565 section 4.1:
    // The AlgorithmIdentifier parameters field MUST be present, and the
    // parameters field MUST contain a AES-IV:
    //
    // AES-IV ::= OCTET STRING (SIZE(16))
    #[defined_by(oid::AES_128_CBC_OID)]
    AesCbc(&'a [u8]),

    // These ECDSA algorithms should have no parameters,
    // but Java 11 (up to at least 11.0.19) encodes them
    // with NULL parameters. The JDK team is looking to
    // backport the fix as of June 2023.
    #[defined_by(oid::ECDSA_WITH_SHA224_OID)]
    EcDsaWithSha224(Option<asn1::Null>),
    #[defined_by(oid::ECDSA_WITH_SHA256_OID)]
    EcDsaWithSha256(Option<asn1::Null>),
    #[defined_by(oid::ECDSA_WITH_SHA384_OID)]
    EcDsaWithSha384(Option<asn1::Null>),
    #[defined_by(oid::ECDSA_WITH_SHA512_OID)]
    EcDsaWithSha512(Option<asn1::Null>),

    #[defined_by(oid::ECDSA_WITH_SHA3_224_OID)]
    EcDsaWithSha3_224,
    #[defined_by(oid::ECDSA_WITH_SHA3_256_OID)]
    EcDsaWithSha3_256,
    #[defined_by(oid::ECDSA_WITH_SHA3_384_OID)]
    EcDsaWithSha3_384,
    #[defined_by(oid::ECDSA_WITH_SHA3_512_OID)]
    EcDsaWithSha3_512,

    #[defined_by(oid::RSA_WITH_SHA1_OID)]
    RsaWithSha1(Option<asn1::Null>),
    #[defined_by(oid::RSA_WITH_SHA1_ALT_OID)]
    RsaWithSha1Alt(Option<asn1::Null>),

    #[defined_by(oid::RSA_WITH_SHA224_OID)]
    RsaWithSha224(Option<asn1::Null>),
    #[defined_by(oid::RSA_WITH_SHA256_OID)]
    RsaWithSha256(Option<asn1::Null>),
    #[defined_by(oid::RSA_WITH_SHA384_OID)]
    RsaWithSha384(Option<asn1::Null>),
    #[defined_by(oid::RSA_WITH_SHA512_OID)]
    RsaWithSha512(Option<asn1::Null>),

    #[defined_by(oid::RSA_WITH_SHA3_224_OID)]
    RsaWithSha3_224(Option<asn1::Null>),
    #[defined_by(oid::RSA_WITH_SHA3_256_OID)]
    RsaWithSha3_256(Option<asn1::Null>),
    #[defined_by(oid::RSA_WITH_SHA3_384_OID)]
    RsaWithSha3_384(Option<asn1::Null>),
    #[defined_by(oid::RSA_WITH_SHA3_512_OID)]
    RsaWithSha3_512(Option<asn1::Null>),

    // RsaPssParameters must be present in Certificate::tbs_cert::signature_alg::params
    // and Certificate::signature_alg::params, but Certificate::tbs_cert::spki::algorithm::oid
    // also uses RSASSA_PSS_OID and the params field is omitted since it has no meaning there.
    #[defined_by(oid::RSASSA_PSS_OID)]
    RsaPss(Option<Box<RsaPssParameters<'a>>>),

    #[defined_by(oid::DSA_OID)]
    Dsa(DssParams<'a>),

    #[defined_by(oid::DSA_WITH_SHA224_OID)]
    DsaWithSha224(Option<asn1::Null>),
    #[defined_by(oid::DSA_WITH_SHA256_OID)]
    DsaWithSha256(Option<asn1::Null>),
    #[defined_by(oid::DSA_WITH_SHA384_OID)]
    DsaWithSha384(Option<asn1::Null>),
    #[defined_by(oid::DSA_WITH_SHA512_OID)]
    DsaWithSha512(Option<asn1::Null>),

    #[defined_by(oid::DH_OID)]
    Dh(DHXParams<'a>),
    #[defined_by(oid::DH_KEY_AGREEMENT_OID)]
    DhKeyAgreement(BasicDHParams<'a>),

    #[default]
    Other(asn1::ObjectIdentifier, Option<asn1::Tlv<'a>>),
}

#[derive(asn1::Asn1Read, asn1::Asn1Write, Hash, PartialEq, Eq, Clone)]
pub struct SubjectPublicKeyInfo<'a> {
    pub algorithm: AlgorithmIdentifier<'a>,
    pub subject_public_key: asn1::BitString<'a>,
}

#[derive(asn1::Asn1Read, asn1::Asn1Write, PartialEq, Eq, Hash, Clone)]
pub struct AttributeTypeValue<'a> {
    pub type_id: asn1::ObjectIdentifier,
    pub value: RawTlv<'a>,
}

// Like `asn1::Tlv` but doesn't store `full_data` so it can be constructed from
// an un-encoded tag and value.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct RawTlv<'a> {
    tag: asn1::Tag,
    value: &'a [u8],
}

impl<'a> RawTlv<'a> {
    pub fn new(tag: asn1::Tag, value: &'a [u8]) -> Self {
        RawTlv { tag, value }
    }

    pub fn tag(&self) -> asn1::Tag {
        self.tag
    }
    pub fn data(&self) -> &'a [u8] {
        self.value
    }
}
impl<'a> asn1::Asn1Readable<'a> for RawTlv<'a> {
    fn parse(parser: &mut asn1::Parser<'a>) -> asn1::ParseResult<Self> {
        let tlv = parser.read_element::<asn1::Tlv<'a>>()?;
        Ok(RawTlv::new(tlv.tag(), tlv.data()))
    }

    fn can_parse(_tag: asn1::Tag) -> bool {
        true
    }
}
impl<'a> asn1::Asn1Writable for RawTlv<'a> {
    fn write(&self, w: &mut asn1::Writer<'_>) -> asn1::WriteResult {
        w.write_tlv(self.tag, move |dest| dest.push_slice(self.value))
    }
}

#[derive(asn1::Asn1Read, asn1::Asn1Write, PartialEq, Eq, Hash, Clone)]
pub enum Time {
    UtcTime(asn1::UtcTime),
    GeneralizedTime(asn1::GeneralizedTime),
}

impl Time {
    pub fn as_datetime(&self) -> &asn1::DateTime {
        match self {
            Time::UtcTime(data) => data.as_datetime(),
            Time::GeneralizedTime(data) => data.as_datetime(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Asn1ReadableOrWritable<T, U> {
    Read(T),
    Write(U),
}

impl<T, U> Asn1ReadableOrWritable<T, U> {
    pub fn new_read(v: T) -> Self {
        Asn1ReadableOrWritable::Read(v)
    }

    pub fn new_write(v: U) -> Self {
        Asn1ReadableOrWritable::Write(v)
    }

    pub fn unwrap_read(&self) -> &T {
        match self {
            Asn1ReadableOrWritable::Read(v) => v,
            Asn1ReadableOrWritable::Write(_) => panic!("unwrap_read called on a Write value"),
        }
    }
}

impl<'a, T: asn1::SimpleAsn1Readable<'a>, U> asn1::SimpleAsn1Readable<'a>
    for Asn1ReadableOrWritable<T, U>
{
    const TAG: asn1::Tag = T::TAG;
    fn parse_data(data: &'a [u8]) -> asn1::ParseResult<Self> {
        Ok(Self::new_read(T::parse_data(data)?))
    }
}

impl<T: asn1::SimpleAsn1Writable, U: asn1::SimpleAsn1Writable> asn1::SimpleAsn1Writable
    for Asn1ReadableOrWritable<T, U>
{
    const TAG: asn1::Tag = U::TAG;
    fn write_data(&self, w: &mut asn1::WriteBuf) -> asn1::WriteResult {
        match self {
            Asn1ReadableOrWritable::Read(v) => T::write_data(v, w),
            Asn1ReadableOrWritable::Write(v) => U::write_data(v, w),
        }
    }
}

#[derive(asn1::Asn1Read, asn1::Asn1Write)]
pub struct DssSignature<'a> {
    pub r: asn1::BigUint<'a>,
    pub s: asn1::BigUint<'a>,
}

#[derive(asn1::Asn1Read, asn1::Asn1Write)]
pub struct DHParams<'a> {
    pub p: asn1::BigUint<'a>,
    pub g: asn1::BigUint<'a>,
    pub q: Option<asn1::BigUint<'a>>,
}

// From PKCS#3 Section 9
// DHParameter ::= SEQUENCE {
//     prime INTEGER, -- p
//     base INTEGER, -- g
//     privateValueLength INTEGER OPTIONAL
// }
#[derive(asn1::Asn1Read, asn1::Asn1Write, Clone, PartialEq, Eq, Debug, Hash)]
pub struct BasicDHParams<'a> {
    pub p: asn1::BigUint<'a>,
    pub g: asn1::BigUint<'a>,
    pub private_value_length: Option<u32>,
}

// From https://www.rfc-editor.org/rfc/rfc3279#section-2.3.3
// DomainParameters ::= SEQUENCE {
//     p       INTEGER, -- odd prime, p=jq +1
//     g       INTEGER, -- generator, g
//     q       INTEGER, -- factor of p-1
//     j       INTEGER OPTIONAL, -- subgroup factor
//     validationParms  ValidationParms OPTIONAL
// }
#[derive(asn1::Asn1Read, asn1::Asn1Write, Clone, PartialEq, Eq, Debug, Hash)]
pub struct DHXParams<'a> {
    pub p: asn1::BigUint<'a>,
    pub g: asn1::BigUint<'a>,
    pub q: asn1::BigUint<'a>,
    pub j: Option<asn1::BigUint<'a>>,
    // No support for this, so don't bother filling out the fields.
    pub validation_params: Option<asn1::Sequence<'a>>,
}

// RSA-PSS ASN.1 default hash algorithm
pub const PSS_SHA1_HASH_ALG: AlgorithmIdentifier<'_> = AlgorithmIdentifier {
    oid: asn1::DefinedByMarker::marker(),
    params: AlgorithmParameters::Sha1(Some(())),
};

// RSA-PSS ASN.1 hash algorithm definitions specified under the CA/B Forum BRs.
pub const PSS_SHA256_HASH_ALG: AlgorithmIdentifier<'_> = AlgorithmIdentifier {
    oid: asn1::DefinedByMarker::marker(),
    params: AlgorithmParameters::Sha256(Some(())),
};

pub const PSS_SHA384_HASH_ALG: AlgorithmIdentifier<'_> = AlgorithmIdentifier {
    oid: asn1::DefinedByMarker::marker(),
    params: AlgorithmParameters::Sha384(Some(())),
};

pub const PSS_SHA512_HASH_ALG: AlgorithmIdentifier<'_> = AlgorithmIdentifier {
    oid: asn1::DefinedByMarker::marker(),
    params: AlgorithmParameters::Sha512(Some(())),
};

// This is defined as an AlgorithmIdentifier in RFC 4055,
// but the mask generation algorithm **must** contain an AlgorithmIdentifier
// in its params, so we define it this way.
#[derive(asn1::Asn1Read, asn1::Asn1Write, Hash, Clone, PartialEq, Eq, Debug)]
pub struct MaskGenAlgorithm<'a> {
    pub oid: asn1::ObjectIdentifier,
    pub params: AlgorithmIdentifier<'a>,
}

// RSA-PSS ASN.1 default mask gen algorithm
pub const PSS_SHA1_MASK_GEN_ALG: MaskGenAlgorithm<'_> = MaskGenAlgorithm {
    oid: oid::MGF1_OID,
    params: PSS_SHA1_HASH_ALG,
};

// RSA-PSS ASN.1 mask gen algorithms defined under the CA/B Forum BRs.
pub const PSS_SHA256_MASK_GEN_ALG: MaskGenAlgorithm<'_> = MaskGenAlgorithm {
    oid: oid::MGF1_OID,
    params: PSS_SHA256_HASH_ALG,
};

pub const PSS_SHA384_MASK_GEN_ALG: MaskGenAlgorithm<'_> = MaskGenAlgorithm {
    oid: oid::MGF1_OID,
    params: PSS_SHA384_HASH_ALG,
};

pub const PSS_SHA512_MASK_GEN_ALG: MaskGenAlgorithm<'_> = MaskGenAlgorithm {
    oid: oid::MGF1_OID,
    params: PSS_SHA512_HASH_ALG,
};

// From RFC 5480 section 2.1.1:
// ECParameters ::= CHOICE {
//     namedCurve         OBJECT IDENTIFIER
//     -- implicitCurve   NULL
//     -- specifiedCurve  SpecifiedECDomain }
//
// Only the namedCurve form may appear in PKIX. Other forms may be found in
// other PKIs.
#[derive(asn1::Asn1Read, asn1::Asn1Write, Hash, Clone, PartialEq, Eq, Debug)]
pub enum EcParameters<'a> {
    NamedCurve(asn1::ObjectIdentifier),
    ImplicitCurve(asn1::Null),
    SpecifiedCurve(asn1::Sequence<'a>),
}

// From RFC 4055 section 3.1:
// RSASSA-PSS-params  ::=  SEQUENCE  {
//     hashAlgorithm      [0] HashAlgorithm DEFAULT
//                               sha1Identifier,
//     maskGenAlgorithm   [1] MaskGenAlgorithm DEFAULT
//                               mgf1SHA1Identifier,
//     saltLength         [2] INTEGER DEFAULT 20,
//     trailerField       [3] INTEGER DEFAULT 1  }
#[derive(asn1::Asn1Read, asn1::Asn1Write, Hash, Clone, PartialEq, Eq, Debug)]
pub struct RsaPssParameters<'a> {
    #[explicit(0)]
    #[default(PSS_SHA1_HASH_ALG)]
    pub hash_algorithm: AlgorithmIdentifier<'a>,
    #[explicit(1)]
    #[default(PSS_SHA1_MASK_GEN_ALG)]
    pub mask_gen_algorithm: MaskGenAlgorithm<'a>,
    #[explicit(2)]
    #[default(20u16)]
    pub salt_length: u16,
    #[explicit(3)]
    #[default(1u8)]
    pub _trailer_field: u8,
}

// https://datatracker.ietf.org/doc/html/rfc3279#section-2.3.2
//
// Dss-Parms ::= SEQUENCE  {
//     p  INTEGER,
//     q  INTEGER,
//     g  INTEGER
// }
#[derive(asn1::Asn1Read, asn1::Asn1Write, Hash, Clone, PartialEq, Eq, Debug)]
pub struct DssParams<'a> {
    pub p: asn1::BigUint<'a>,
    pub q: asn1::BigUint<'a>,
    pub g: asn1::BigUint<'a>,
}

/// A VisibleString ASN.1 element whose contents is not validated as meeting the
/// requirements (visible characters of IA5), and instead is only known to be
/// valid UTF-8.
pub struct UnvalidatedVisibleString<'a>(pub &'a str);

impl<'a> UnvalidatedVisibleString<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl<'a> asn1::SimpleAsn1Readable<'a> for UnvalidatedVisibleString<'a> {
    const TAG: asn1::Tag = asn1::VisibleString::TAG;
    fn parse_data(data: &'a [u8]) -> asn1::ParseResult<Self> {
        Ok(UnvalidatedVisibleString(
            std::str::from_utf8(data)
                .map_err(|_| asn1::ParseError::new(asn1::ParseErrorKind::InvalidValue))?,
        ))
    }
}

impl<'a> asn1::SimpleAsn1Writable for UnvalidatedVisibleString<'a> {
    const TAG: asn1::Tag = asn1::VisibleString::TAG;
    fn write_data(&self, _: &mut asn1::WriteBuf) -> asn1::WriteResult {
        unimplemented!();
    }
}

/// A BMPString ASN.1 element, where it is stored as a UTF-8 string in memory.
pub struct Utf8StoredBMPString<'a>(pub &'a str);

impl<'a> Utf8StoredBMPString<'a> {
    pub fn new(s: &'a str) -> Self {
        Utf8StoredBMPString(s)
    }
}

impl<'a> asn1::SimpleAsn1Writable for Utf8StoredBMPString<'a> {
    const TAG: asn1::Tag = asn1::BMPString::TAG;
    fn write_data(&self, writer: &mut asn1::WriteBuf) -> asn1::WriteResult {
        for ch in self.0.encode_utf16() {
            writer.push_slice(&ch.to_be_bytes())?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct WithTlv<'a, T> {
    tlv: asn1::Tlv<'a>,
    value: T,
}

impl<'a, T> WithTlv<'a, T> {
    pub fn tlv(&self) -> &asn1::Tlv<'a> {
        &self.tlv
    }
}

impl<T> std::ops::Deref for WithTlv<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T: asn1::Asn1Readable<'a>> asn1::Asn1Readable<'a> for WithTlv<'a, T> {
    fn parse(p: &mut asn1::Parser<'a>) -> asn1::ParseResult<Self> {
        let tlv = p.read_element::<asn1::Tlv<'a>>()?;
        Ok(Self {
            tlv,
            value: tlv.parse()?,
        })
    }

    fn can_parse(t: asn1::Tag) -> bool {
        T::can_parse(t)
    }
}

impl<'a, T: asn1::Asn1Writable> asn1::Asn1Writable for WithTlv<'a, T> {
    fn write(&self, w: &mut asn1::Writer<'_>) -> asn1::WriteResult<()> {
        self.value.write(w)
    }
}

impl<T: PartialEq> PartialEq for WithTlv<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T: Eq> Eq for WithTlv<'_, T> {}
impl<T: std::hash::Hash> std::hash::Hash for WithTlv<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use asn1::Asn1Readable;

    use super::{Asn1ReadableOrWritable, RawTlv, UnvalidatedVisibleString, WithTlv};

    #[test]
    #[should_panic]
    fn test_unvalidated_visible_string_write() {
        let v = UnvalidatedVisibleString("foo");
        asn1::write_single(&v).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_asn1_readable_or_writable_unwrap_read() {
        Asn1ReadableOrWritable::<u32, u32>::new_write(17).unwrap_read();
    }

    #[test]
    fn test_asn1_readable_or_writable_write_read_data() {
        let v = Asn1ReadableOrWritable::<u32, u32>::new_read(17);
        assert_eq!(&asn1::write_single(&v).unwrap(), b"\x02\x01\x11");
    }

    #[test]
    fn test_raw_tlv_can_parse() {
        let t = asn1::Tag::from_bytes(&[0]).unwrap().0;
        assert!(RawTlv::can_parse(t));
    }

    #[test]
    fn test_with_raw_tlv_can_parse() {
        let t = asn1::Tag::from_bytes(&[0x30]).unwrap().0;

        assert!(WithTlv::<asn1::Sequence<'_>>::can_parse(t));
        assert!(!WithTlv::<bool>::can_parse(t));
    }
}
