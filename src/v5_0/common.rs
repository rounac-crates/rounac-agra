#![doc = r#"Module with basic types."#]

use serde::{Deserialize, Serialize};

#[doc = r#"Specifies which AA Codes will be interrogated for the BDS Registers in the next field. If omitted, implies all targets.

# Restrictions
* Length: `6`"#]
pub type AaCodeType = String;

#[doc = r#"Indicates acceleration in meters per second per second (m/s^2)."#]
pub type AccelerationType = f64;

#[doc = r#"String representing AIMIDB flight numbers.

# Restrictions
* Pattern: `[A-Z0-9]{1}[0-9 ]{2}`
* Length: `3`"#]
pub type AimidbFlightNumberStringType = String;

#[doc = r#"String representing an AIMIDB mission number.

# Restrictions
* Pattern: `(((((U0)|[A-Z]{2})[0-9]{2})|UNKN) )`
* Length: `5`"#]
pub type AimidbMissionNoType = String;

#[doc = r#"Air Defense District (ADD) or Air Defense Area (ADA) in which the geographic coordinates reside. ([A-Z][A-Z] - Position 1-2, Two character alphabetic field.; 0[0-9][0-9] - Position 3-5,  Air Defense Area in which the geographic coordinates resides.).

# Restrictions
* Pattern: `[A-Z]{2}[0][0-9]{2}`
* Length: `5`"#]
pub type AirDefenseAreaType = String;

#[doc = r#"Provides the Mode S Aircraft ID. See MIL-STD-6016 for detailed definition.

# Restrictions
* Pattern: `[A-Z0-9 ]{8}`
* Length: `8`"#]
pub type AircraftIdentifierType = String;

#[doc = r#"A 4 character string that provides the ICAO (International Civil Aviation Organization) identifier for an airfield (DFI 1868 DUI 001).

# Restrictions
* Pattern: `[A-Z]{4}`
* Length: `4`"#]
pub type AirfieldIdIcaoStringType = String;

#[doc = r#"String type of exactly 10 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{10}`
* Length: `10`"#]
pub type AlphanumericDashSpaceUnderscoreString10OnlyType = String;

#[doc = r#"String type of exactly 11 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{11}`
* Length: `11`"#]
pub type AlphanumericDashSpaceUnderscoreString11OnlyType = String;

#[doc = r#"String type of up to 11 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{1,11}`
* Minimum length: `1`
* Maximum length: `11`"#]
pub type AlphanumericDashSpaceUnderscoreString11Type = String;

#[doc = r#"String type of exactly 12 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{12}`
* Length: `12`"#]
pub type AlphanumericDashSpaceUnderscoreString12OnlyType = String;

#[doc = r#"String type of exactly 13 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{13}`
* Length: `13`"#]
pub type AlphanumericDashSpaceUnderscoreString13OnlyType = String;

#[doc = r#"String type of up to 13 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{1,13}`
* Minimum length: `1`
* Maximum length: `13`"#]
pub type AlphanumericDashSpaceUnderscoreString13Type = String;

#[doc = r#"String type of exactly 15 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{15}`
* Length: `15`"#]
pub type AlphanumericDashSpaceUnderscoreString15OnlyType = String;

#[doc = r#"String type of exactly 16 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{16}`
* Length: `16`"#]
pub type AlphanumericDashSpaceUnderscoreString16OnlyType = String;

#[doc = r#"String type of up to 16 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{1,16}`
* Minimum length: `1`
* Maximum length: `16`"#]
pub type AlphanumericDashSpaceUnderscoreString16Type = String;

#[doc = r#"String type of exactly 17 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{17}`
* Length: `17`"#]
pub type AlphanumericDashSpaceUnderscoreString17OnlyType = String;

#[doc = r#"String type of exactly 20 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{20}`
* Length: `20`"#]
pub type AlphanumericDashSpaceUnderscoreString20OnlyType = String;

#[doc = r#"String type of up to 20 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type AlphanumericDashSpaceUnderscoreString20Type = String;

#[doc = r#"String type of exactly 21 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{21}`
* Length: `21`"#]
pub type AlphanumericDashSpaceUnderscoreString21OnlyType = String;

#[doc = r#"String type of exactly 3 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{3}`
* Length: `3`"#]
pub type AlphanumericDashSpaceUnderscoreString3OnlyType = String;

#[doc = r#"String type of up to 3 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{1,3}`
* Minimum length: `1`
* Maximum length: `3`"#]
pub type AlphanumericDashSpaceUnderscoreString3Type = String;

#[doc = r#"String type of up to 40 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{1,40}`
* Minimum length: `1`
* Maximum length: `40`"#]
pub type AlphanumericDashSpaceUnderscoreString40Type = String;

#[doc = r#"String type of exactly 56 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{56}`
* Length: `56`"#]
pub type AlphanumericDashSpaceUnderscoreString56OnlyType = String;

#[doc = r#"String type of exactly 9 characters in length, restricted to alphanumeric characters, dash, space, and underscore.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{9}`
* Length: `9`"#]
pub type AlphanumericDashSpaceUnderscoreString9OnlyType = String;

#[doc = r#"String type of up to 11 characters in length, restricted to alphanumeric and punctuation characters.

# Restrictions
* Pattern: `[A-Za-z0-9 ]{1,10}[!-~]`
* Minimum length: `1`
* Maximum length: `11`"#]
pub type AlphanumericPunctuatedString11Type = String;

#[doc = r#"String type of exactly 11 characters in length, restricted to alphanumeric characters and space.

# Restrictions
* Pattern: `[A-Za-z0-9 ]{11}`
* Length: `11`"#]
pub type AlphanumericSpaceString11OnlyType = String;

#[doc = r#"String type of up to 11 characters in length, restricted to alphanumeric characters and space.

# Restrictions
* Pattern: `[A-Za-z0-9 ]{1,11}`
* Minimum length: `1`
* Maximum length: `11`"#]
pub type AlphanumericSpaceString11Type = String;

#[doc = r#"String type of exactly 13 characters in length, restricted to alphanumeric characters and space.

# Restrictions
* Pattern: `[A-Za-z0-9 ]{13}`
* Length: `13`"#]
pub type AlphanumericSpaceString13OnlyType = String;

#[doc = r#"String type of exactly 15 characters in length, restricted to alphanumeric characters and space.

# Restrictions
* Pattern: `[A-Za-z0-9 ]{15}`
* Length: `15`"#]
pub type AlphanumericSpaceString15OnlyType = String;

#[doc = r#"String type of exactly 16 characters in length, restricted to alphanumeric characters and space.

# Restrictions
* Pattern: `[A-Za-z0-9 ]{16}`
* Length: `16`"#]
pub type AlphanumericSpaceString16OnlyType = String;

#[doc = r#"String type of exactly 7 characters in length, restricted to alphanumeric characters and space.

# Restrictions
* Pattern: `[A-Za-z0-9 ]{7}`
* Length: `7`"#]
pub type AlphanumericSpaceString7OnlyType = String;

#[doc = r#"String type of exactly 4 characters in length, restricted to alphanumeric characters.

# Restrictions
* Pattern: `[A-Za-z0-9]{4}`
* Length: `4`"#]
pub type AlphanumericString4OnlyType = String;

#[doc = r#"String type of up to 54 characters in length, restricted to alphanumeric characters.

# Restrictions
* Pattern: `[a-zA-Z0-9]{1,54}`
* Maximum length: `54`"#]
pub type AlphanumericString54Type = String;

#[doc = r#"String type of up to 6 characters in length, restricted to alphanumeric characters.

# Restrictions
* Pattern: `[A-Za-z0-9]{1,6}`
* Minimum length: `1`
* Maximum length: `6`"#]
pub type AlphanumericString6Type = String;

#[doc = r#"String type of exactly 7 characters in length, restricted to alphanumeric characters.

# Restrictions
* Pattern: `[A-Za-z0-9]{7}`
* Length: `7`"#]
pub type AlphanumericString7OnlyType = String;

#[doc = r#"Indicates height above Mean Sea Level (MSL) as measured by local barometric pressure in meters (m). The minimum value represents the maximum distance to the center of the EGM96 Geoid from MSL.

# Restrictions
* Minimum value: `-6378237` (Inclusive)"#]
pub type AltitudeBarometricType = f64;

#[doc = r#"Indicates height above reference in meters (m). Where such reference is unavailable, defaults to Height above WGS-84 ellipsoid in meters. The minimum value represents the maximum distance to the center of the EGM96 Geoid from MSL, and also includes the distance to the center of the WGS84 ellipsoid from the equator [Ref DMA TR 8350].

# Restrictions
* Minimum value: `-6378237` (Inclusive)"#]
pub type AltitudeType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [0,Pi].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `3.141592653589793238462` (Inclusive)"#]
pub type AngleHalfPositiveType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [-Pi/2,Pi/2].

# Restrictions
* Minimum value: `-1.570796326794896619232` (Inclusive)
* Maximum value: `1.570796326794896619232` (Inclusive)"#]
pub type AngleHalfType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [0,2*Pi).

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `6.283185307179586476926` (Inclusive)"#]
pub type AnglePositiveType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [0,Pi/2].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `1.570796326794896619232` (Inclusive)"#]
pub type AngleQuarterType = f64;

#[doc = r#"Indicates an angle rate in radians/sec (rad/s)."#]
pub type AngleRateType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [-Pi,Pi).

# Restrictions
* Minimum value: `-3.141592653589793238462` (Inclusive)
* Maximum value: `3.141592653589793238462` (Inclusive)"#]
pub type AngleType = f64;

#[doc = r#"4 digit Pulse Internal Modulation Code.

# Restrictions
* Pattern: `[0-9]{4}`
* Length: `4`"#]
pub type AoPimCodeType = String;

#[doc = r#"Joint Pub 3-09.1 defines the PRF Code to be: "...Depending on the laser equipment, either a three or four-digit code can be set. Three digit code equipment settings range from 111 to 788. Four-digit code equipment settings range from 1111 to 1788.

# Restrictions
* Pattern: `1?[1-7][1-8]{2}`
* Minimum length: `3`
* Maximum length: `4`"#]
pub type AoPrfCodeType = String;

#[doc = r#"Indicates an angular measurement in units of arc seconds (arcsec)."#]
pub type ArcSecondsType = f64;

#[doc = r#"Indicates a non-negative area in square meters (m^2)."#]
pub type AreaType = DoubleNonNegativeType;

#[doc = r#"Provides the name of a resource or location.

# Restrictions
* Pattern: `[ -~]{1,256}`
* Minimum length: `1`
* Maximum length: `256`"#]
pub type AttributedUriType = String;

#[doc = r#"Surveillance Identifier (modern address type): lockout interactions

# Restrictions
* Length: `2`"#]
pub type BdsAddressType = String;

#[doc = r#"Indicates 8-bit integer values greater than or equal to one with a range of [1,255].

# Restrictions
* Minimum value: `1` (Inclusive)"#]
pub type BytePositiveType = u8;

#[doc = r#"String type of exactly 2 characters in length, restricted to a pair of letters or a pair of spaces.

# Restrictions
* Pattern: `([A-Z]{2}|[ ]{2})`
* Length: `2`"#]
pub type CharOrSpacePairsType = String;

#[doc = r#"Indicates the identifier of the crypto slot within a group of crypto key slots.  This is used as a unique identifier for the crypto slot."#]
pub type CryptoSlotIdentifierType = VisibleString32Type;

#[doc = r#"Cycle number format.

# Restrictions
* Pattern: `[A-Z0-9]{2}-[0-9]{4}[TE]?`
* Minimum length: `7`
* Maximum length: `8`"#]
pub type CsCycleNumberType = String;

#[doc = r#"Indicates the System Engagement Number (SENO) for a CounterSpace force.

# Restrictions
* Pattern: `[A-Z][IRS][0-9]{3}`
* Length: `5`"#]
pub type CsSenoType = String;

#[doc = r#"Indicates a data rate in bits per seconds (bps)."#]
pub type DataRateType = u32;

#[doc = r#"UCI uses the W3C (www.w3.org) definition of date and time exactly as given in the specification for xs:dateTime with a further restriction that only the "Zulu" time zone be used.  xs:dateTime is based on Coordinated Universal Time (UTC) and allows seconds to be specified with decimal digits to arbitrary precision.  See the W3C specification of xs:dateTime for further details.

# Restrictions
* Pattern: `.+Z`"#]
pub type DateTimeType = i64;

#[doc = r#"Indicates non-negative values in Decibels (dB).

# Restrictions
* Minimum value: `0` (Inclusive)"#]
pub type DecibelNonNegativeType = DecibelType;

#[doc = r#"Indicates values in Decibels (dB)."#]
pub type DecibelType = f64;

#[doc = r#"Indicates a distance offset from an origin in 1-dimension, in meters (m)."#]
pub type DistanceOffsetType = f64;

#[doc = r#"Indicates a (non-negative) distance in meters (m)."#]
pub type DistanceType = DoubleNonNegativeType;

#[doc = r#"Indicates 64-bit floating point values greater than or equal to zero [0,Inf].

# Restrictions
* Minimum value: `0.0` (Inclusive)"#]
pub type DoubleNonNegativeType = f64;

#[doc = r#"Indicates 64-bit floating point values greater than zero (0,Inf].

# Restrictions
* Minimum value: `0.0` (Exclusive)"#]
pub type DoublePositiveType = f64;

#[doc = r#"UCI uses the W3C (www.w3.org) definition of time duration exactly as given in the specification for xs:duration.  xs:duration is based on Coordinated Universal Time (UTC) and allows seconds to be specified with decimal digits to arbitrary precision.  See the W3C specification of xs:duration for further details."#]
pub type DurationType = i64;

#[doc = r#"Indicates the terrain level relative to Mean Sea Level (MSL) in meters (m)."#]
pub type ElevationType = f64;

#[doc = r#"An empty, blank type used to indicate that no further content is expected.

# Restrictions
* Pattern: `[A-Za-z]{0}`
* Length: `0`"#]
pub type EmptyType = String;

#[doc = r#"Indicates the rate at which energy is dissipated, measured in watts per kilogram (W/kg)."#]
pub type EnergyDissipationRateType = DoubleNonNegativeType;

#[doc = r#"Indicates the entity's function or mission that it may or may not be engaged in at the moment. Based on the Global Command and Control System Integrated Imagery and Intelligence (GCCS-I3) Application Program Interface Reference Manual (APIRM) for MIDB Data Access Layer (MDAL).

# Restrictions
* Pattern: `[A-Z0-9]{1,3}`
* Minimum length: `1`
* Maximum length: `3`"#]
pub type EobActivityCodeType = String;

#[doc = r#"Indicates the emitter's name in the Combined Emitter Database (CED). See the Combined Emitter Database (CED) for more information.

# Restrictions
* Pattern: `[A-Za-z0-9_\-]{0,12}`
* Minimum length: `0`
* Maximum length: `12`"#]
pub type EobCedNameType = String;

#[doc = r#"The description of an EOB Code Word may be found in the classified appendix of the MIDB. Based on the Global Command and Control System Integrated Imagery and Intelligence (GCCS-I3) Application Program Interface Reference Manual (APIRM) for MIDB Data Access Layer (MDAL).

# Restrictions
* Pattern: `[A-Za-z0-9_\-]{0,5}`
* Minimum length: `0`
* Maximum length: `5`"#]
pub type EobCodeWordType = String;

#[doc = r#"Indicates the surrogate key, which uniquely identifies the emitter in the source MIDB. Based on the Global Command and Control System Integrated Imagery and Intelligence (GCCS-I3) Application Program Interface Reference Manual (APIRM) for MIDB Data Access Layer (MDAL).

# Restrictions
* Pattern: `[0-9A-Za-z]{5}[0-9]{9}`
* Maximum length: `14`"#]
pub type EobEmitterSurrogateKeyType = String;

#[doc = r#"Identifies the EOB site facility or demographic area.  Typically used in conjunction with other data, such as a Basic Encyclopedia Number, to uniquely identify an EOB site.

# Restrictions
* Pattern: `[A-Z]{2}[0-9]{3}`
* Length: `5`"#]
pub type EobOSuffixType = String;

#[doc = r#"Indicates the name of the EOB as a facility or populated area.  Typically used in conjunction with other data to uniquely identify an EOB.

# Restrictions
* Pattern: `[A-Za-z0-9_\-]{0,54}`
* Minimum length: `0`
* Maximum length: `54`"#]
pub type EobSiteNameType = String;

#[doc = r#"Indicates the EOB's associated weapon system based on a correlation with the Combined Emitter Database. Reference the Combined Emitter Database (CED) for more information.

# Restrictions
* Pattern: `[A-Za-z0-9_\-]{0,20}`
* Minimum length: `0`
* Maximum length: `20`"#]
pub type EobWeaponSystemType = String;

#[doc = r#"String representing the authority used to classify the file.

# Restrictions
* Pattern: `[ODM ]{1,2}`
* Minimum length: `1`
* Maximum length: `2`"#]
pub type FileHeaderClassificationAuthorityType = String;

#[doc = r#"String representing what classification a file may be downgraded to.

# Restrictions
* Pattern: `[SCR ]{1,2}`
* Minimum length: `1`
* Maximum length: `2`"#]
pub type FileHeaderDowngradeStringType = String;

#[doc = r#"This element defines the file name of the stored product.  The minimum naming convention of file names is {at least one alphanumeric, underscore or dash} {dot} { at least one alphanumeric/underscore/dash or a dot}. The format supports multiple dot extensions, such as "filename.tar.bz2".  The max length value of 255 is set to be consistent with  NTFS, ext4, btrfs, and zfs restrictions.

# Restrictions
* Pattern: `[a-zA-Z0-9_-]+\.[a-zA-Z0-9_.-]+`
* Maximum length: `255`"#]
pub type FileNameType = String;

#[doc = r#"String representing a FIPS classification.

# Restrictions
* Pattern: `[A-Z ]{2}[A-Z ]?`
* Minimum length: `2`
* Maximum length: `3`"#]
pub type FipsClassificationSystemType = String;

#[doc = r#"Indicates a frequency offset in Hertz (Hz)."#]
pub type FrequencyOffsetType = f64;

#[doc = r#"Indicates a (positive) frequency in Hertz (Hz)."#]
pub type FrequencyType = DoublePositiveType;

#[doc = r#"Indicates the daily average level for geomagnetic activity, represented as the Ap Index.

# Restrictions
* Maximum value: `400` (Inclusive)"#]
pub type GeomagneticApIndexType = DoubleNonNegativeType;

#[doc = r#"Indicates the planetary amplitude of gamma deflections (geomagnetic activity), represented as the Kp Index.

# Restrictions
* Maximum value: `9` (Inclusive)"#]
pub type GeomagneticKpIndexType = DoubleNonNegativeType;

#[doc = r#"Indicates an ICAO Aircraft Address with a range of [0,16777215].

# Restrictions
* Maximum value: `16777215` (Inclusive)"#]
pub type IcaoAircraftAddressType = u32;

#[doc = r#"These are from the IFF specification and identify 7 possible Subtypes to the Types in ADS-B.

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `7` (Inclusive)"#]
pub type IffAdsBSubtypeType = u8;

#[doc = r#"These are from the IFF specification and identify 31 possible formats for Mode5

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `31` (Inclusive)"#]
pub type IffAdsBType = u8;

#[doc = r#"These are from the IFF specification and identify 31 possible formats for Mode5

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `31` (Inclusive)"#]
pub type IffMode5FormatType = u8;

#[doc = r#"IJMS 15 bit Track Number.

# Restrictions
* Pattern: `[0-7]{5}`
* Length: `5`"#]
pub type IjmsTrackNumberType = String;

#[doc = r#"String representing an IMO number.

# Restrictions
* Pattern: `IMO[0-9]{7}`
* Length: `10`"#]
pub type ImoNumberType = String;

#[doc = r#"Indicates 32-bit integer values greater than zero with a range of [1,4294967295].

# Restrictions
* Minimum value: `1` (Inclusive)"#]
pub type IntPositiveType = u32;

#[doc = r#"Interrogator ID (legacy address type): used for transponder lockout interactions

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `15` (Inclusive)"#]
pub type InterrogatorIdentifierType = i32;

#[doc = r#"String representing an IPON IID2 Program Code.

# Restrictions
* Pattern: `([0-9][A-Z]|[A-Z][0-9])`
* Length: `2`"#]
pub type IponIid2ProgramCodeType = String;

#[doc = r#"String representing an IPON IID2 Project Code.

# Restrictions
* Pattern: `([A-Z][A-Z])`
* Length: `2`"#]
pub type IponIid2ProjectCodeType = String;

#[doc = r#"String representing an IPON IID2 Sortie number.

# Restrictions
* Pattern: `([0-9A-Z]{2})`
* Length: `2`"#]
pub type IponIid2SortieNumberType = String;

#[doc = r#"String representation of an IPv4 address, with validation rules derived from RFC 6991.

# Restrictions
* Pattern: `(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])`
* Minimum length: `7`
* Maximum length: `15`"#]
pub type Ipv4AddressType = String;

#[doc = r#"String representation of an IPv6 address, with validation rules derived from RFC 6991.

# Restrictions
* Pattern: `((:|[0-9a-fA-F]{0,4}):)([0-9a-fA-F]{0,4}:){0,5}((([0-9a-fA-F]{0,4}:)?(:|[0-9a-fA-F]{0,4}))|(((25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])))`
* Minimum length: `2`
* Maximum length: `45`"#]
pub type Ipv6AddressType = String;

#[doc = r#"Indicates the measured irradiance of the object being observed in Watts per square meter (W/m^2)."#]
pub type IrradianceType = DoubleNonNegativeType;

#[doc = r#"Indicates the object's launch piece (A-Z) within a launch number.  This is part of the unique designation for an Earth orbiting object assigned by the World Data Center-A for Rockets and Satellites (WDC-A-RS).

# Restrictions
* Pattern: `[0-9a-zA-Z]{1,3}`
* Minimum length: `1`
* Maximum length: `3`"#]
pub type LaunchPieceType = String;

#[doc = r#"Link 11 12 bit Track Number.

# Restrictions
* Pattern: `[0-7]{4}`
* Length: `4`"#]
pub type Link11TrackNumberType = String;

#[doc = r#"Indicates a Link-16 Control Channel with a range of [0,127].

# Restrictions
* Maximum value: `127` (Inclusive)"#]
pub type Link16ControlChannelType = u8;

#[doc = r#"Indicates a Link-16 Message Generation Rate with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16MessageGenerationRateType = u8;

#[doc = r#"Indicates a Link-16 J-Message Label with a range of [0,31].

# Restrictions
* Maximum value: `31` (Inclusive)"#]
pub type Link16MessageLabelType = u8;

#[doc = r#"Indicates a Link-16 Message Priority with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16MessagePriorityType = u8;

#[doc = r#"Indicates a number of messages that can be stored in a queue with a range of [0,280].

# Restrictions
* Maximum value: `280` (Inclusive)"#]
pub type Link16MessageStorageLimitType = u16;

#[doc = r#"Indicates a Link-16 J-Message Sub-Label with a range of [0,7].

# Restrictions
* Maximum value: `7` (Inclusive)"#]
pub type Link16MessageSubLabelType = u8;

#[doc = r#"Indicates a Link-16 Missile Channel with a range of [1,63]. See MIL-STD-6016 DFI 852 DUI 002.

# Restrictions
* Maximum value: `63` (Inclusive)"#]
pub type Link16MissileChannelType = BytePositiveType;

#[doc = r#"Indicates a Link-16 Net Number with a range of [0,127].

# Restrictions
* Maximum value: `127` (Inclusive)"#]
pub type Link16NetNumberType = u8;

#[doc = r#"Indicates a Link-16 Network Participation Group with a range of [0,512].

# Restrictions
* Maximum value: `512` (Inclusive)"#]
pub type Link16NpgType = u16;

#[doc = r#"Indicates a Link-16 Participation Group Index number with a range of [0,511].

# Restrictions
* Maximum value: `511` (Inclusive)"#]
pub type Link16ParticipationGroupIndexType = u16;

#[doc = r#"Indicates a Link-16 Platform Strength with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16PlatformStrengthType = u8;

#[doc = r#"Indicates a Link-16 Position Quality with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16PositionQualityType = u8;

#[doc = r#"Indicates a Link-16 Radar RF channel with a range of [1,63]. See MIL-STD-6016 DFI 852 DUI 001.

# Restrictions
* Maximum value: `63` (Inclusive)"#]
pub type Link16RadarChannelType = BytePositiveType;

#[doc = r#"Indicates a Link-16 Slot Number with a range of [1,10].

# Restrictions
* Maximum value: `10` (Inclusive)"#]
pub type Link16SlotNumberType = BytePositiveType;

#[doc = r#"Indicates a Link-16 Slot Selection with a range of [0,31].

# Restrictions
* Maximum value: `31` (Inclusive)"#]
pub type Link16SlotSelectionType = u8;

#[doc = r#"Indicates a number of Link-16 time slots before a message is considered stale with a range of [1,16383].

# Restrictions
* Maximum value: `16383` (Inclusive)"#]
pub type Link16StalenessLimitType = ShortPositiveType;

#[doc = r#"Indicates a Link-16 track index for a specific entity member with a range of [0,63]. See MIL-STD-6016 DFI 768 DUI 002.

# Restrictions
* Maximum value: `63` (Inclusive)"#]
pub type Link16TrackIndexType = u8;

#[doc = r#"Indicates a Link-16 Track Number with a range of [0,32767].

# Restrictions
* Maximum value: `32767` (Inclusive)"#]
pub type Link16TrackNumberType = u16;

#[doc = r#"Indicates a Link-16 Track Quality with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16TrackQualityType = u8;

#[doc = r#"Indicates a Link-16 Voice Channel with a range of [0,127].

# Restrictions
* Maximum value: `127` (Inclusive)"#]
pub type Link16VoiceChannelType = u8;

#[doc = r#"Slot identifier for system's formation position.

# Restrictions
* Minimum value: `0` (Inclusive)"#]
pub type MaFormationSlotType = i32;

#[doc = r#"Indicates the Mach value, which is unitless and represents the ratio of the speed of a body to the speed of sound in the surrounding medium."#]
pub type MachType = DoubleNonNegativeType;

#[doc = r#"Indicates mass in kilograms (kg)."#]
pub type MassType = DoubleNonNegativeType;

#[doc = r#"String type restricted to the letter M.

# Restrictions
* Pattern: `[m]`
* Length: `1`"#]
pub type MeterUnitLetterStringType = String;

#[doc = r#"Indicates an MIDB Evaluation Code with a range of [1,10].

# Restrictions
* Maximum value: `10` (Inclusive)"#]
pub type MidbEvaluationCodeType = BytePositiveType;

#[doc = r#"String used to represent a military grid.

# Restrictions
* Pattern: `([1-9]|[1-5][0-9]|60)[C-HJ-NP-X]([A-HJ-NP-Z][A-HJ-NP-V]([0-9]{2}){0,5})?|[ABYZ]([A-CF-HJ-LP-UX-Z][A-HJ-NP-Z]([0-9]{2}){0,5})?`
* Minimum length: `14`
* Maximum length: `15`"#]
pub type MilitaryGridStringType = String;

#[doc = r#"Indicates power referenced to one milliwatt in units dBm."#]
pub type MilliwattPowerRatioType = f64;

#[doc = r#"This MIME type describes the product's physical encoding."#]
pub type MimeType = VisibleString256Type;

#[doc = r#"Indicates the purpose or category of the mission as specified by an operator.  Used to provide the ability to filter missions by their type."#]
pub type MissionCategoryType = VisibleString32Type;

#[doc = r#"String representing a MMSI number.

# Restrictions
* Pattern: `[0-9]{9}`
* Length: `9`"#]
pub type MmsiNumberStringType = String;

#[doc = r#"Indicates the 2 digit Mode 1 code.

# Restrictions
* Pattern: `[0-7]{2}`
* Length: `2`"#]
pub type Mode1CodeType = String;

#[doc = r#"Indicates the 4 digit Mode 1 code.

# Restrictions
* Pattern: `[0-7]{4}`
* Length: `4`"#]
pub type Mode1FourCharacterCodeType = String;

#[doc = r#"String representing an unsignedInt with 5 significant figures.

# Restrictions
* Pattern: `[1-9][0-9 ]{5}`
* Length: `6`"#]
pub type MstgtaTgtCatStringType = String;

#[doc = r#"String representing a MSTGTA TGT LOC.

# Restrictions
* Pattern: `([\+-]{1}[0-8]\d\.\d{6}[\+-]{1}(0\d{2}|1[0-7]\d)\.\d{6})`
* Length: `21`"#]
pub type MstgtaTgtLocType = String;

#[doc = r#"String representing a MSTGTA TGT LTIOV.

# Restrictions
* Pattern: `(([12]\d\d\d)((0[1-9])|(1[012]))(0[1-9]|[12][0-9]|3[01])([01][0-9]|[2][0-3])([0-5][0-9]))`
* Length: `12`"#]
pub type MstgtaTgtLtiovType = String;

#[doc = r#"String representing an unsignedInt time with 6 significant figures.

# Restrictions
* Pattern: `[0-9]{6}Z `
* Length: `8`"#]
pub type MstgtaTgtUtcType = String;

#[doc = r#"Official name of something, or object that represents it. Special characters are restricted to apostrophe ('), at sign (@), parenthesis (), comma (,), period (.), semicolon (;), plus sign (+), and dash (-).  These are needed to enable automated data exchange with other systems.).

# Restrictions
* Pattern: `[a-zA-Z0-9 \-]{15}`
* Length: `15`"#]
pub type NameSpecialCharacterRestrictionType = String;

#[doc = r#"NATO Link 1 15-bit Track Number.

# Restrictions
* Pattern: `[AEGHJKLM]{2}[0-7]{3}`
* Length: `5`"#]
pub type NatoLink1TrackNumberType = String;

#[doc = r#"North Atlantic Treaty Organization Special WordsTotal number of distinct values is 54.North Atlantic Treaty Organization Special Words

# Restrictions
* Pattern: `NATO/[a-zA-Z\-_]`
* Length: `6`"#]
pub type NatoSpecialWordsType = String;

#[doc = r#"Indicates a desired/observed rating level of an image in the National Imagery Interpretability Rating Scale.  Scale levels are expressed as digit, dot, digit, like "3.2" with a range 0.0 to 9.9.  NIIRS defines "scales" (with levels and associated rating criteria) for images of different types and/or collected in different bands of the electromagnetic spectrum.  These scales include Visible NIIRS (visible panchromatic, military focused), Civil NIIRS (visible panchromatic, civil focused), Radar NIIRS (synthetic aperture radar), IR NIIRS (infrared/thermal), MS IIRS (multispectral) and MT IIRS (moving target radar).

# Restrictions
* Pattern: `[0-9]\.[0-9]`
* Length: `3`"#]
pub type NiirsType = String;

#[doc = r#"String representing NITF classification authority.

# Restrictions
* Pattern: `[ODM ]`
* Length: `1`"#]
pub type NitfClassificationAuthorityType = String;

#[doc = r#"String representing NITF classification reason.

# Restrictions
* Pattern: `[A-G ]?`
* Length: `1`"#]
pub type NitfClassificationReasonType = String;

#[doc = r#"String representing NITF codewords.

# Restrictions
* Pattern: `(([A-Z]{2}( [A-Z]{2}){0,3})|( ){11})`
* Minimum length: `2`
* Maximum length: `11`"#]
pub type NitfCodewordsType = String;

#[doc = r#"String representing NITF control and handling.

# Restrictions
* Pattern: `([ -~ -ÿ]{2})`
* Length: `2`"#]
pub type NitfControlAndHandlingType = String;

#[doc = r#"See simple type documentation. NITF date format of CCYYMMDD, used in various NITF data fields. Default of eight ECS spaces (0x20) indicates that the date does not imply.

# Restrictions
* Pattern: `(([12]\d\d\d((0[1-9])|(1[012]))(0[1-9]|[12][0-9]|3[01]))|( ){8})`
* Length: `8`"#]
pub type NitfDateType = String;

#[doc = r#"String representing NITF declassification and exemptions.

# Restrictions
* Pattern: `(X[1-8]|X25[1-9]|( ){4})`
* Minimum length: `2`
* Maximum length: `4`"#]
pub type NitfDeclassificationExemptionType = String;

#[doc = r#"String representing NITF file declassification.

# Restrictions
* Pattern: `(DD|DE|GD|GE|O|X|( ){2})`
* Maximum length: `2`"#]
pub type NitfDeclassificationType = String;

#[doc = r#"String representing NITF downgrades.

# Restrictions
* Pattern: `[SCR ]`
* Length: `1`"#]
pub type NitfDowngradeType = String;

#[doc = r#"String representing NITF file security classification.

# Restrictions
* Pattern: `[TSCRU]`
* Length: `1`"#]
pub type NitfFileSecurityClassificationType = String;

#[doc = r#"String representing NITF releasing instructions.

# Restrictions
* Pattern: `(([A-Z]{2}( [A-Z]{2}){0,6})|( ){20})`
* Minimum length: `2`
* Maximum length: `20`"#]
pub type NitfReleasingInstructionsType = String;

#[doc = r#"Indicates a NITF Target Functional Category Code with a range of [10000,99999]. See DIAM-65-3-1.

# Restrictions
* Minimum value: `10000` (Inclusive)
* Maximum value: `99999` (Inclusive)"#]
pub type NitfTargetCategoryCodeType = i32;

#[doc = r#"Indicates a NITF Target Number with a range of [1,99999].

# Restrictions
* Maximum value: `99999` (Inclusive)"#]
pub type NitfTargetNumberType = IntPositiveType;

#[doc = r#"Indicates a NITF Target Priority with a range of [1,999].

# Restrictions
* Maximum value: `999` (Inclusive)"#]
pub type NitfTargetPriorityType = ShortPositiveType;

#[doc = r#"CVEnumISMNonIC PatternsThe name of the ALTERNATE COMPENSATORY CONTROL MEASURE, substituting "_" for a space.

# Restrictions
* Pattern: `ACCM-[A-Z0-9\-_]{1,61}`
* Minimum length: `6`
* Maximum length: `66`"#]
pub type NonIcMarkingsType = String;

#[doc = r#"Indicates non IPON identifier.

# Restrictions
* Pattern: `[ -~ -ÿ]{1,80}`
* Minimum length: `1`
* Maximum length: `80`"#]
pub type NonIponIid2Type = String;

#[doc = r#"A string representing the notation for ELNOT or CENOT. ELNOT is Electronic Intelligence (ELINT) Notation for non-communications electronic emissions. CENOT is Communications Emitter Notation for communications electronic emissions.

# Restrictions
* Pattern: `[A-Z0-9]{5}|UNKN|NONE`
* Minimum length: `4`
* Maximum length: `5`"#]
pub type NotationType = String;

#[doc = r#"String type of up to 11 characters in length, restricted to numeric characters and space.

# Restrictions
* Pattern: `[0-9 ]{1,11}`
* Minimum length: `1`
* Maximum length: `11`"#]
pub type NumericSpaceString11Type = String;

#[doc = r#"String type of exactly 13 characters in length, restricted to numeric characters and space.

# Restrictions
* Pattern: `[0-9 ]{13}`
* Length: `13`"#]
pub type NumericSpaceString13OnlyType = String;

#[doc = r#"String type of exactly 2 characters in length, restricted to numeric characters and space.

# Restrictions
* Pattern: `[0-9 ]{2}`
* Length: `2`"#]
pub type NumericSpaceString2OnlyType = String;

#[doc = r#"String type of exactly 4 characters in length, restricted to numeric characters and space.

# Restrictions
* Pattern: `[0-9 ]{4}`
* Length: `4`"#]
pub type NumericSpaceString4OnlyType = String;

#[doc = r#"String type of exactly 6 characters in length, restricted to numeric characters and space.

# Restrictions
* Pattern: `[0-9 ]{6}`
* Length: `6`"#]
pub type NumericSpaceString6OnlyType = String;

#[doc = r#"String type of exactly 7 characters in length, restricted to numeric characters and space.

# Restrictions
* Pattern: `[0-9 ]{7}`
* Length: `7`"#]
pub type NumericSpaceString7OnlyType = String;

#[doc = r#"Type containing an octal number in the form of a string.

# Restrictions
* Pattern: `[0-7]+`
* Maximum length: `16`"#]
pub type OctalValueType = String;

#[doc = r#"Indicates the presence of clouds in the sky in units of okta with a range of [0,8].

# Restrictions
* Maximum value: `8` (Inclusive)"#]
pub type OktaType = u8;

#[doc = r#"String representing an Encyclopedia agency code.

# Restrictions
* Pattern: `[0-9]{4,5}`
* Minimum length: `4`
* Maximum length: `5`"#]
pub type OneUpNumberStringType = String;

#[doc = r#"Indicates up to a 20-digit US phone number. The operator may list several phone numbers including extensions and the string can include dots,hyphens and/or spaces.  Example formats include: 555-555-5555x5555, 5555555555, 555 555 5555x555555.

# Restrictions
* Pattern: `([0-9a-zA-Z\s\-\.]){1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type OperatorPhoneNumberType = String;

#[doc = r#"Indicates the name of the source of the last collection of the subject data for the order of battle item.

# Restrictions
* Pattern: `[A-Z0-9]{2}[A-Z0-9]?`
* Maximum length: `3`"#]
pub type OrderOfBattleLastCollectorType = String;

#[doc = r#"Indicates the name of the creator and/or owner of the master instance of the order of battle item.

# Restrictions
* Pattern: `[A-Za-z0-9 \-_]{0,3}`
* Minimum length: `0`
* Maximum length: `3`"#]
pub type OrderOfBattleRecordOwnerType = String;

#[doc = r#"Non-negative double value indicating particles per cubic centimeter."#]
pub type ParticleDensityType = DoubleNonNegativeType;

#[doc = r#"String representing the floating point value of Gravity at a local point with 8 significant figures.

# Restrictions
* Pattern: `[0-9\. ]{8}`
* Length: `8`"#]
pub type PatchbGravityStringType = String;

#[doc = r#"Indicates a percentage where a value of 100.0 = 100%. Values greater than 100 are allowed."#]
pub type PercentType = DoubleNonNegativeType;

#[doc = r#"This type stores a power in Watts."#]
pub type PowerType = f64;

#[doc = r#"Indicates precipitation amounts in millimeters (mm)."#]
pub type PrecipitationAmountType = DoubleNonNegativeType;

#[doc = r#"Indicates pressure in kiloPascal (kPa)."#]
pub type PressureType = f64;

#[doc = r#"String type of 4096 characters in length. All characters are allowed for maximum compatibility with other string restrictions.

# Restrictions
* Pattern: `(.|&#10;|&#13;){0,4096}`
* Minimum length: `0`
* Maximum length: `4096`"#]
pub type QueryString4096Type = String;

#[doc = r#"String representing an Encyclopedia record originator.

# Restrictions
* Pattern: `[A-Z][A-Z]|[E]|[\-]`
* Minimum length: `1`
* Maximum length: `2`"#]
pub type RecordOriginatorStringType = String;

#[doc = r#"Indicates resolution in meters per pixel."#]
pub type ResolutionMetersPerPixelType = DoubleNonNegativeType;

#[doc = r#"Indicates the identity of the ROME ID, which is defined by ACTDF, where ROME is the acronym for Reconnaissance Operations Management Enterprise.

# Restrictions
* Pattern: `[a-zA-Z0-9]+`
* Maximum length: `10`"#]
pub type RomeIdentityType = String;

#[doc = r#"(U) All currently valid SAR controls from the published register
						 PERMISSIBLE VALUES

						 The permissible values for this simple type are defined in the Controlled Value Enumeration:

						 CVEnumISMSAR.xmlSPECIAL ACCESS REQUIRED-XXX,Within the nickname or name of a SAR all spaces must be replaced with a "_". The XSL will restore the spaces for rendering.SPECIAL ACCESS REQUIRED-XXX, the Digraph or Trigraph of the SAR is represented by the XXXSPECIAL ACCESS REQUIRED-XXX, the Digraph or Trigraph of the SAR is represented by the XXXSPECIAL ACCESS REQUIRED-XXX, the Digraph or Trigraph of the SAR is represented by the XXX

# Restrictions
* Pattern: `[A-Z]{2,}-[A-Z][A-Z0-9]+-[A-Z0-9]{2,}`
* Minimum length: `1`
* Maximum length: `100`"#]
pub type SarIdentifierType = String;

#[doc = r#"The schema component name or enumeration value as it appears in the schema.  The UCI Style and Design Specification restricts these values to the alphanumeric, underscore, and dash characters.

# Restrictions
* Pattern: `[a-zA-Z0-9_-]{1,128}`
* Minimum length: `1`
* Maximum length: `128`"#]
pub type SchemaComponentNameType = String;

#[doc = r#"CVEnumISMSCIControls PatternsKDK-BLFH-xxxxxx, xxxxxx represents up to 6 alphanumeric characters indicating a sub BLUEFISH compartmentKDK-IDIT-xxxxxx, xxxxxx represents up to 6 alphanumeric characters indicating a sub IDITAROD compartmentKDK-KAND-xxxxxx, xxxxxx represents up to 6 alphanumeric characters indicating a sub KANDIK compartmentRSV-XXX, XXX represents 3 alpha numeric characters to indicate sub Reserve compartmentsG-AAAA, AAAA represents 4 alpha characters to indicate sub Gamma compartmentsSPECIAL INTELLIGENCE compartmentSPECIAL INTELLIGENCE sub-compartment

# Restrictions
* Pattern: `SI-[A-Z]{3}-[A-Z]{4}`
* Minimum length: `6`
* Maximum length: `15`"#]
pub type SciControlsType = String;

#[doc = r#"This type indicates a SHA-256 cryptographic hash as defined by the U.S. Federal Information Processing Standard (FIPS) Publication 180-2.  SHA-256 is one of a family of Secure Hash Algorithms (SHA) published by the National Institute of Standards and Technology (NIST).  The 256 bit hash is encoded here as a hexadecimal number.

# Restrictions
* Length: `32`"#]
pub type Sha2256HashType = String;

#[doc = r#"Indicates 16-bit integer values greater than zero with a range of [1,65535].

# Restrictions
* Minimum value: `1` (Inclusive)"#]
pub type ShortPositiveType = u16;

#[doc = r#"String type of exactly 1 character in length, restricted to a numeric character.

# Restrictions
* Pattern: `[0-9]`
* Length: `1`"#]
pub type SingleDigitStringType = String;

#[doc = r#"Indicates F10.7 solar flux index."#]
pub type SolarFluxF107IndexType = u16;

#[doc = r#"Indicates a special code for this vehicle. The actual meaning of the value is defined by the current operation.

# Restrictions
* Pattern: `[0-7]{4}`
* Length: `4`"#]
pub type SpecialCode2Type = String;

#[doc = r#"The model designation suffix of a specific type of vehicle. Indicates the alpha character suffix used to designate a certain version of an alpha/numeric aircraft designator.  Use of a single character indicates the MIL-STD-6016 aircraft version; see MIL-STD-6016 DFI/DUI 1661/001 for additional details.  Use of a longer string is program dependent.

# Restrictions
* Pattern: `[A-Za-z0-9]{4}`
* Length: `4`"#]
pub type SpecificTypeModelType = String;

#[doc = r#"Indicates speed in meters per second (m/s)."#]
pub type SpeedType = f64;

#[doc = r#"Indicates a scale factor with a range of [1.0,Inf].

# Restrictions
* Minimum value: `1.0` (Inclusive)"#]
pub type SpoilFactorType = f32;

#[doc = r#"Surface Moving Target Indicator nationality from STANAG 4607 Edition 3 Field P3.

# Restrictions
* Pattern: `[A-Za-z0-9]{0,2}`
* Minimum length: `0`
* Maximum length: `2`"#]
pub type Stanag4607NationalityType = String;

#[doc = r#"String representing a STANAG packet security classification.

# Restrictions
* Pattern: `[1-5]`
* Length: `1`"#]
pub type Stanag4607PacketSecurityClassificationType = String;

#[doc = r#"String representing a STANAG 4607 plan.

# Restrictions
* Pattern: `([ -~&#10;&#13;]{1,12})`
* Minimum length: `1`
* Maximum length: `12`"#]
pub type Stanag4607PlanType = String;

#[doc = r#"Surface Moving Target Indicator platform ID from STANAG 4607 Edition 3 Field P8.

# Restrictions
* Pattern: `[A-Za-z0-9]{0,10}`
* Minimum length: `0`
* Maximum length: `10`"#]
pub type Stanag4607PlatformIdentifierType = String;

#[doc = r#"String type of zero to 20 characters in length with no restrictions on the legal characters.

# Restrictions
* Pattern: `[A-Za-z0-9]{0,20}`
* Minimum length: `0`
* Maximum length: `20`"#]
pub type String20Type = String;

#[doc = r#"String type of zero to 4 characters in length with no restrictions on the legal characters.

# Restrictions
* Pattern: `[A-Za-z0-9]{0,4}`
* Minimum length: `0`
* Maximum length: `4`"#]
pub type String4Type = String;

#[doc = r#"Surveillance Identifier (modern address type): lockout interactions

# Restrictions
* Minimum value: `1` (Inclusive)
* Maximum value: `63` (Inclusive)"#]
pub type SurveillanceIdentifierType = i32;

#[doc = r#"Indicates the vehicle's tail number.  Typically used for aircraft vehicles.  DoD aircraft are identified by a five-digit tail number. For most military aircraft, the first two digits of the tail number are the fiscal year that the aircraft was ordered, and the remaining digits are the last three digits of the airframe's serial number. US Civil aircraft are identified by a two to six character alphanumeric registration number assigned by the ICAO or FAA. International tail numbers (registrations) follow similar patterns, but may be up to 10 characters long, often containing a dash separating the country code from the specific alphanumeric registration.

# Restrictions
* Pattern: `[A-Z0-9\-]{2,10}`
* Minimum length: `2`
* Maximum length: `10`"#]
pub type TailNumberType = String;

#[doc = r#"Indicates temperature in degrees Celsius (C)."#]
pub type TemperatureType = f64;

#[doc = r#"String representing time in NITF. Used for arrival time to target.

# Restrictions
* Pattern: `(([01][0-9]|[2][0-3])([0-5][0-9])([0-5][0-9])Z)`
* Length: `7`"#]
pub type TgtUtcstringType = String;

#[doc = r#"UCI uses the W3C (www.w3.org) definition of time exactly as given in the specification for xs:time with a further restriction that only the "Zulu" time zone be used.  xs:time is based on Coordinated Universal Time (UTC) and allows seconds to be specified with decimal digits to arbitrary precision.  See the W3C specification of xs:time for further details.

# Restrictions
* Pattern: `.+Z`"#]
pub type TimeType = i64;

#[doc = r#"String representing the UCI version.

# Restrictions
* Pattern: `[0-9]{3}\.[0-9]{1,2}([a-z]{1})?(\.[0-9A-Za-z_\-]{1,45})?`
* Minimum length: `5`
* Maximum length: `53`"#]
pub type UciSchemaVersionStringType = String;

#[doc = r#"Indicates 64-bit floating point values with a range of [-1,1].

# Restrictions
* Minimum value: `-1` (Inclusive)
* Maximum value: `1` (Inclusive)"#]
pub type UnitBallDoubleType = f64;

#[doc = r#"Indicates 32-bit floating point values with a range of [-1,1].

# Restrictions
* Minimum value: `-1.0` (Inclusive)
* Maximum value: `1.0` (Inclusive)"#]
pub type UnitBallFloatType = f32;

#[doc = r#"Unique identifier for each unit. ([A-Z][A-Z] Position 1-2, SYSTEM ASSIGNED RECORD ORIGINATOR.  Two character code associated with the organization originating the unique UNIT ID.  Assigned by system at element creation time.     [ABCDEGJMNSX] Position 3, OB_TYPE     A Air Force     B Joint Forces     C Civilian     D Defensive Missile Forces     E Net     G Army     J Space Order of Battle (SOB)     M Ministry, Other Than Ministry of Defense (MOD)     N Navy     S Strategic Missile Forces     X Air Defense Order of Battle (ADOB)     [A-Z][A-Z] Position 4-5, ALLEGIANCE.  This item is selected from the 2 character list of valid State Department Allegiance codes.     [00001-99999] Position 6-10, ACCESSION_NUMBER.).

# Restrictions
* Pattern: `[A-Z0-9]{10}`
* Length: `10`"#]
pub type UnitIdentifierType = String;

#[doc = r#"Indicates 64-bit floating point values with a range of [0,1].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `1.0` (Inclusive)"#]
pub type UnitIntervalDoubleType = f64;

#[doc = r#"Indicates 32-bit floating point values with a range of [0,1].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `1.0` (Inclusive)"#]
pub type UnitIntervalFloatType = f32;

#[doc = r#"Translated unit name or identification given the unit by appropriate authority or orders as used in official orders or communications within the national military or civilian establishment of the country of allegiance. A unit name must be established for every unit in the database.  For each Unit logical record, unit naming conventions established in production programs should be employed.  If official sources are not available, the unit name believed most correct is used.  A unit's primary designation usually includes service specialty and command echelon.

# Restrictions
* Pattern: `[a-zA-Z0-9 '\(\).,@;+\-]*`"#]
pub type UnitNameType = VisibleString256Type;

#[doc = r#"A UUID is a 128-bit number (32 hexadecimal digits, 16 bytes) that is conformant to a version and variant of IETF RFC 4122.

# Restrictions
* Length: `16`"#]
pub type UniversallyUniqueIdentifierType = String;

#[doc = r#"The ID type for UCI IDs that correspond to system users.

# Restrictions
* Pattern: `[0-9a-zA-Z]{1,25}`
* Minimum length: `1`
* Maximum length: `25`"#]
pub type UserIdentifierType = String;

#[doc = r#"String type of up to 15 characters in length, restricted to ASCII visible characters (0x20-0x7E) and the Unicode Latin 1 Supplement (0xA0-0xFF).

# Restrictions
* Pattern: `([ -~ -ÿ]{1,15})`
* Minimum length: `1`
* Maximum length: `15`"#]
pub type VisibleLatin1String15Type = String;

#[doc = r#"String type of up to 18 characters in length, restricted to ASCII visible characters (0x20-0x7E) and the Unicode Latin 1 Supplement (0xA0-0xFF).

# Restrictions
* Pattern: `([ -~ -ÿ]{1,18})`
* Minimum length: `1`
* Maximum length: `18`"#]
pub type VisibleLatin1String18Type = String;

#[doc = r#"String type of up to 24 characters in length, restricted to ASCII visible characters (0x20-0x7E) and the Unicode Latin 1 Supplement (0xA0-0xFF).

# Restrictions
* Pattern: `([ -~ -ÿ]{1,24})`
* Minimum length: `1`
* Maximum length: `24`"#]
pub type VisibleLatin1String24Type = String;

#[doc = r#"String type of up to 40 characters in length, restricted to ASCII visible characters (0x20-0x7E) and the Unicode Latin 1 Supplement (0xA0-0xFF).

# Restrictions
* Pattern: `([ -~ -ÿ]{1,40})`
* Minimum length: `1`
* Maximum length: `40`"#]
pub type VisibleLatin1String40Type = String;

#[doc = r#"String type of up to 43 characters in length, restricted to ASCII visible characters (0x20-0x7E) and the Unicode Latin 1 Supplement (0xA0-0xFF).

# Restrictions
* Pattern: `([ -~ -ÿ]{1,43})`
* Minimum length: `1`
* Maximum length: `43`"#]
pub type VisibleLatin1String43Type = String;

#[doc = r#"String type of 1024 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{0,1024}`
* Minimum length: `0`
* Maximum length: `1024`"#]
pub type VisibleString1024Type = String;

#[doc = r#"String type of up to 10 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,10}`
* Minimum length: `1`
* Maximum length: `10`"#]
pub type VisibleString10Type = String;

#[doc = r#"String type of 128 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{0,128}`
* Minimum length: `0`
* Maximum length: `128`"#]
pub type VisibleString128Type = String;

#[doc = r#"String type of up to 12 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `([ -~]{1,12})`
* Maximum length: `12`"#]
pub type VisibleString12Type = String;

#[doc = r#"String type of up to 15 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `([ -~]{1,15})`
* Minimum length: `1`
* Maximum length: `15`"#]
pub type VisibleString15Type = String;

#[doc = r#"String type of up to 16 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,16}`
* Minimum length: `1`
* Maximum length: `16`"#]
pub type VisibleString16Type = String;

#[doc = r#"String type of up to 17 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `([ -~]{1,17})`
* Minimum length: `1`
* Maximum length: `17`"#]
pub type VisibleString17Type = String;

#[doc = r#"String type of up to 20 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type VisibleString20Type = String;

#[doc = r#"String type between 2 and 4 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{2,4}`
* Minimum length: `2`
* Maximum length: `4`"#]
pub type VisibleString24Type = String;

#[doc = r#"String type of 256 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{0,256}`
* Minimum length: `0`
* Maximum length: `256`"#]
pub type VisibleString256Type = String;

#[doc = r#"String type of 32 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{0,32}`
* Minimum length: `0`
* Maximum length: `32`"#]
pub type VisibleString32Type = String;

#[doc = r#"String type of up to 3 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,3}`
* Minimum length: `1`
* Maximum length: `3`"#]
pub type VisibleString3Type = String;

#[doc = r#"String type of up to 480 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,480}`
* Minimum length: `1`
* Maximum length: `480`"#]
pub type VisibleString480Type = String;

#[doc = r#"String type of 512 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{0,512}`
* Minimum length: `0`
* Maximum length: `512`"#]
pub type VisibleString512Type = String;

#[doc = r#"String type of 64 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{0,64}`
* Minimum length: `0`
* Maximum length: `64`"#]
pub type VisibleString64Type = String;

#[doc = r#"String type of up to 80 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,80}`
* Minimum length: `1`
* Maximum length: `80`"#]
pub type VisibleString80Type = String;

#[doc = r#"String type of up to 81 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,81}`
* Minimum length: `1`
* Maximum length: `81`"#]
pub type VisibleString81Type = String;

#[doc = r#"String type of up to 9 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[ -~]{1,9}`
* Minimum length: `1`
* Maximum length: `9`"#]
pub type VisibleString9Type = String;

#[doc = r#"Indicates the brightness of an observed object."#]
pub type VisualMagnitudeType = f64;

#[doc = r#"The Link 16 Voice Call Sign used to identify this vehicle.

# Restrictions
* Pattern: `[ 0-9A-Z]{4}`
* Length: `4`"#]
pub type VoiceCallSignType = String;

#[doc = r#"String type of 1024 characters in length, restricted to visible characters (0x21-0x7E) and whitespace characters.

# Restrictions
* Pattern: `[\s&#x21;-&#x7E;]{0,1024}`
* Minimum length: `0`
* Maximum length: `1024`"#]
pub type WhitespaceVisibleString1024Type = String;

#[doc = r#"String type of 4096 characters in length, restricted to visible characters (0x21-0x7E) and whitespace characters.

# Restrictions
* Pattern: `[\s&#x21;-&#x7E;]{0,4096}`
* Minimum length: `0`
* Maximum length: `4096`"#]
pub type WhitespaceVisibleString4096Type = String;

