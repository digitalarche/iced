/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::fmt;

// GENERATOR-BEGIN: DisplSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum DisplSize {
	None,
	Size1,
	Size2,
	Size4,
	Size8,
	RipRelSize4_Target32,
	RipRelSize4_Target64,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_DISPL_SIZE: [&str; 7] = [
	"None",
	"Size1",
	"Size2",
	"Size4",
	"Size8",
	"RipRelSize4_Target32",
	"RipRelSize4_Target64",
];
impl fmt::Debug for DisplSize {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_DISPL_SIZE[*self as usize])?;
		Ok(())
	}
}
impl Default for DisplSize {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		DisplSize::None
	}
}
// GENERATOR-END: DisplSize

// GENERATOR-BEGIN: ImmSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum ImmSize {
	None,
	Size1,
	Size2,
	Size4,
	Size8,
	/// `ENTER xxxx,yy`
	Size2_1,
	/// `EXTRQ/INSERTQ xx,yy`
	Size1_1,
	/// `CALL16 FAR x:y`
	Size2_2,
	/// `CALL32 FAR x:y`
	Size4_2,
	RipRelSize1_Target16,
	RipRelSize1_Target32,
	RipRelSize1_Target64,
	RipRelSize2_Target16,
	RipRelSize2_Target32,
	RipRelSize2_Target64,
	RipRelSize4_Target32,
	RipRelSize4_Target64,
	SizeIbReg,
	Size1OpCode,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_IMM_SIZE: [&str; 19] = [
	"None",
	"Size1",
	"Size2",
	"Size4",
	"Size8",
	"Size2_1",
	"Size1_1",
	"Size2_2",
	"Size4_2",
	"RipRelSize1_Target16",
	"RipRelSize1_Target32",
	"RipRelSize1_Target64",
	"RipRelSize2_Target16",
	"RipRelSize2_Target32",
	"RipRelSize2_Target64",
	"RipRelSize4_Target32",
	"RipRelSize4_Target64",
	"SizeIbReg",
	"Size1OpCode",
];
impl fmt::Debug for ImmSize {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_IMM_SIZE[*self as usize])?;
		Ok(())
	}
}
impl Default for ImmSize {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		ImmSize::None
	}
}
// GENERATOR-END: ImmSize

// GENERATOR-BEGIN: EncoderFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EncoderFlags;
impl EncoderFlags {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const B: u32 = 0x0000_0001;
	pub(crate) const X: u32 = 0x0000_0002;
	pub(crate) const R: u32 = 0x0000_0004;
	pub(crate) const W: u32 = 0x0000_0008;
	pub(crate) const MOD_RM: u32 = 0x0000_0010;
	pub(crate) const SIB: u32 = 0x0000_0020;
	pub(crate) const REX: u32 = 0x0000_0040;
	pub(crate) const P66: u32 = 0x0000_0080;
	pub(crate) const P67: u32 = 0x0000_0100;
	/// `EVEX.R'`
	pub(crate) const R2: u32 = 0x0000_0200;
	pub(crate) const BROADCAST: u32 = 0x0000_0400;
	pub(crate) const HIGH_LEGACY_8_BIT_REGS: u32 = 0x0000_0800;
	pub(crate) const DISPL: u32 = 0x0000_1000;
	pub(crate) const PF0: u32 = 0x0000_2000;
	pub(crate) const VVVVV_SHIFT: u32 = 0x0000_001B;
	pub(crate) const VVVVV_MASK: u32 = 0x0000_001F;
}
// GENERATOR-END: EncoderFlags

// GENERATOR-BEGIN: OperandSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum OperandSize {
	None,
	Size16,
	Size32,
	Size64,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_OPERAND_SIZE: [&str; 4] = [
	"None",
	"Size16",
	"Size32",
	"Size64",
];
impl fmt::Debug for OperandSize {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OPERAND_SIZE[*self as usize])?;
		Ok(())
	}
}
impl Default for OperandSize {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		OperandSize::None
	}
}
// GENERATOR-END: OperandSize

// GENERATOR-BEGIN: AddressSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum AddressSize {
	None,
	Size16,
	Size32,
	Size64,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_ADDRESS_SIZE: [&str; 4] = [
	"None",
	"Size16",
	"Size32",
	"Size64",
];
impl fmt::Debug for AddressSize {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_ADDRESS_SIZE[*self as usize])?;
		Ok(())
	}
}
impl Default for AddressSize {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		AddressSize::None
	}
}
// GENERATOR-END: AddressSize

// GENERATOR-BEGIN: LegacyOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum LegacyOpCodeTable {
	Normal,
	Table0F,
	Table0F38,
	Table0F3A,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_LEGACY_OP_CODE_TABLE: [&str; 4] = [
	"Normal",
	"Table0F",
	"Table0F38",
	"Table0F3A",
];
impl fmt::Debug for LegacyOpCodeTable {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_LEGACY_OP_CODE_TABLE[*self as usize])?;
		Ok(())
	}
}
impl Default for LegacyOpCodeTable {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		LegacyOpCodeTable::Normal
	}
}
// GENERATOR-END: LegacyOpCodeTable

// GENERATOR-BEGIN: VexOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum VexOpCodeTable {
	Table0F = 1,
	Table0F38,
	Table0F3A,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_VEX_OP_CODE_TABLE: [&str; 3] = [
	"Table0F",
	"Table0F38",
	"Table0F3A",
];
impl fmt::Debug for VexOpCodeTable {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_VEX_OP_CODE_TABLE[*self as usize])?;
		Ok(())
	}
}
impl Default for VexOpCodeTable {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		VexOpCodeTable::Table0F
	}
}
// GENERATOR-END: VexOpCodeTable

// GENERATOR-BEGIN: XopOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum XopOpCodeTable {
	XOP8,
	XOP9,
	XOPA,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_XOP_OP_CODE_TABLE: [&str; 3] = [
	"XOP8",
	"XOP9",
	"XOPA",
];
impl fmt::Debug for XopOpCodeTable {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_XOP_OP_CODE_TABLE[*self as usize])?;
		Ok(())
	}
}
impl Default for XopOpCodeTable {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		XopOpCodeTable::XOP8
	}
}
// GENERATOR-END: XopOpCodeTable

// GENERATOR-BEGIN: EvexOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum EvexOpCodeTable {
	Table0F = 1,
	Table0F38,
	Table0F3A,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_EVEX_OP_CODE_TABLE: [&str; 3] = [
	"Table0F",
	"Table0F38",
	"Table0F3A",
];
impl fmt::Debug for EvexOpCodeTable {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_EVEX_OP_CODE_TABLE[*self as usize])?;
		Ok(())
	}
}
impl Default for EvexOpCodeTable {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		EvexOpCodeTable::Table0F
	}
}
// GENERATOR-END: EvexOpCodeTable

// GENERATOR-BEGIN: Encodable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum Encodable {
	Any,
	Only1632,
	Only64,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_ENCODABLE: [&str; 3] = [
	"Any",
	"Only1632",
	"Only64",
];
impl fmt::Debug for Encodable {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_ENCODABLE[*self as usize])?;
		Ok(())
	}
}
impl Default for Encodable {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		Encodable::Any
	}
}
// GENERATOR-END: Encodable

// GENERATOR-BEGIN: VexVectorLength
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum VexVectorLength {
	LZ,
	L0,
	L1,
	L128,
	L256,
	LIG,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_VEX_VECTOR_LENGTH: [&str; 6] = [
	"LZ",
	"L0",
	"L1",
	"L128",
	"L256",
	"LIG",
];
impl fmt::Debug for VexVectorLength {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_VEX_VECTOR_LENGTH[*self as usize])?;
		Ok(())
	}
}
impl Default for VexVectorLength {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		VexVectorLength::LZ
	}
}
// GENERATOR-END: VexVectorLength

// GENERATOR-BEGIN: XopVectorLength
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum XopVectorLength {
	L128,
	L256,
	L0,
	L1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_XOP_VECTOR_LENGTH: [&str; 4] = [
	"L128",
	"L256",
	"L0",
	"L1",
];
impl fmt::Debug for XopVectorLength {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_XOP_VECTOR_LENGTH[*self as usize])?;
		Ok(())
	}
}
impl Default for XopVectorLength {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		XopVectorLength::L128
	}
}
// GENERATOR-END: XopVectorLength

// GENERATOR-BEGIN: EvexVectorLength
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum EvexVectorLength {
	L128,
	L256,
	L512,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_EVEX_VECTOR_LENGTH: [&str; 3] = [
	"L128",
	"L256",
	"L512",
];
impl fmt::Debug for EvexVectorLength {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_EVEX_VECTOR_LENGTH[*self as usize])?;
		Ok(())
	}
}
impl Default for EvexVectorLength {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		EvexVectorLength::L128
	}
}
// GENERATOR-END: EvexVectorLength

// GENERATOR-BEGIN: EncFlags1
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EncFlags1;
impl EncFlags1 {
	pub(crate) const ENCODING_SHIFT: u32 = 0x0000_0000;
	pub(crate) const ENCODING_MASK: u32 = 0x0000_0007;
	pub(crate) const OP_CODE_SHIFT: u32 = 0x0000_0010;
}
// GENERATOR-END: EncFlags1

// GENERATOR-BEGIN: LegacyFlags3
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct LegacyFlags3;
impl LegacyFlags3 {
	pub(crate) const OP_MASK: u32 = 0x0000_007F;
	pub(crate) const OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const OP1_SHIFT: u32 = 0x0000_0007;
	pub(crate) const OP2_SHIFT: u32 = 0x0000_000E;
	pub(crate) const OP3_SHIFT: u32 = 0x0000_0015;
}
// GENERATOR-END: LegacyFlags3

// GENERATOR-BEGIN: VexFlags3
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct VexFlags3;
impl VexFlags3 {
	pub(crate) const OP_MASK: u32 = 0x0000_003F;
	pub(crate) const OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const OP1_SHIFT: u32 = 0x0000_0006;
	pub(crate) const OP2_SHIFT: u32 = 0x0000_000C;
	pub(crate) const OP3_SHIFT: u32 = 0x0000_0012;
	pub(crate) const OP4_SHIFT: u32 = 0x0000_0018;
}
// GENERATOR-END: VexFlags3

// GENERATOR-BEGIN: XopFlags3
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct XopFlags3;
impl XopFlags3 {
	pub(crate) const OP_MASK: u32 = 0x0000_001F;
	pub(crate) const OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const OP1_SHIFT: u32 = 0x0000_0005;
	pub(crate) const OP2_SHIFT: u32 = 0x0000_000A;
	pub(crate) const OP3_SHIFT: u32 = 0x0000_000F;
}
// GENERATOR-END: XopFlags3

// GENERATOR-BEGIN: EvexFlags3
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EvexFlags3;
impl EvexFlags3 {
	pub(crate) const OP_MASK: u32 = 0x0000_003F;
	pub(crate) const OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const OP1_SHIFT: u32 = 0x0000_0006;
	pub(crate) const OP2_SHIFT: u32 = 0x0000_000C;
	pub(crate) const OP3_SHIFT: u32 = 0x0000_0012;
}
// GENERATOR-END: EvexFlags3

// GENERATOR-BEGIN: AllowedPrefixes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum AllowedPrefixes {
	None,
	Bnd,
	BndNotrack,
	HintTakenBnd,
	Lock,
	Rep,
	RepRepne,
	XacquireXreleaseLock,
	Xrelease,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_ALLOWED_PREFIXES: [&str; 9] = [
	"None",
	"Bnd",
	"BndNotrack",
	"HintTakenBnd",
	"Lock",
	"Rep",
	"RepRepne",
	"XacquireXreleaseLock",
	"Xrelease",
];
impl fmt::Debug for AllowedPrefixes {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_ALLOWED_PREFIXES[*self as usize])?;
		Ok(())
	}
}
impl Default for AllowedPrefixes {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		AllowedPrefixes::None
	}
}
// GENERATOR-END: AllowedPrefixes

// GENERATOR-BEGIN: WBit
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum WBit {
	W0,
	W1,
	WIG,
	WIG32,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_WBIT: [&str; 4] = [
	"W0",
	"W1",
	"WIG",
	"WIG32",
];
impl fmt::Debug for WBit {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_WBIT[*self as usize])?;
		Ok(())
	}
}
impl Default for WBit {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		WBit::W0
	}
}
// GENERATOR-END: WBit

// GENERATOR-BEGIN: LegacyFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct LegacyFlags;
impl LegacyFlags {
	pub(crate) const MANDATORY_PREFIX_BYTE_MASK: u32 = 0x0000_0003;
	pub(crate) const MANDATORY_PREFIX_BYTE_SHIFT: u32 = 0x0000_0000;
	pub(crate) const LEGACY_OP_CODE_TABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const LEGACY_OP_CODE_TABLE_SHIFT: u32 = 0x0000_0002;
	pub(crate) const ENCODABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const ENCODABLE_SHIFT: u32 = 0x0000_0004;
	pub(crate) const HAS_GROUP_INDEX: u32 = 0x0000_0040;
	pub(crate) const GROUP_SHIFT: u32 = 0x0000_0007;
	pub(crate) const ALLOWED_PREFIXES_MASK: u32 = 0x0000_000F;
	pub(crate) const ALLOWED_PREFIXES_SHIFT: u32 = 0x0000_000A;
	pub(crate) const FWAIT: u32 = 0x0000_4000;
	pub(crate) const HAS_MANDATORY_PREFIX: u32 = 0x0000_8000;
	pub(crate) const OPERAND_SIZE_MASK: u32 = 0x0000_0003;
	pub(crate) const OPERAND_SIZE_SHIFT: u32 = 0x0000_0010;
	pub(crate) const ADDRESS_SIZE_MASK: u32 = 0x0000_0003;
	pub(crate) const ADDRESS_SIZE_SHIFT: u32 = 0x0000_0012;
}
// GENERATOR-END: LegacyFlags

// GENERATOR-BEGIN: VexFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct VexFlags;
impl VexFlags {
	pub(crate) const MANDATORY_PREFIX_BYTE_MASK: u32 = 0x0000_0003;
	pub(crate) const MANDATORY_PREFIX_BYTE_SHIFT: u32 = 0x0000_0000;
	pub(crate) const VEX_OP_CODE_TABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const VEX_OP_CODE_TABLE_SHIFT: u32 = 0x0000_0002;
	pub(crate) const ENCODABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const ENCODABLE_SHIFT: u32 = 0x0000_0004;
	pub(crate) const HAS_GROUP_INDEX: u32 = 0x0000_0040;
	pub(crate) const GROUP_SHIFT: u32 = 0x0000_0007;
	pub(crate) const VEX_VECTOR_LENGTH_MASK: u32 = 0x0000_0007;
	pub(crate) const VEX_VECTOR_LENGTH_SHIFT: u32 = 0x0000_000A;
	pub(crate) const WBIT_MASK: u32 = 0x0000_0003;
	pub(crate) const WBIT_SHIFT: u32 = 0x0000_000D;
}
// GENERATOR-END: VexFlags

// GENERATOR-BEGIN: XopFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct XopFlags;
impl XopFlags {
	pub(crate) const MANDATORY_PREFIX_BYTE_MASK: u32 = 0x0000_0003;
	pub(crate) const MANDATORY_PREFIX_BYTE_SHIFT: u32 = 0x0000_0000;
	pub(crate) const XOP_OP_CODE_TABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const XOP_OP_CODE_TABLE_SHIFT: u32 = 0x0000_0002;
	pub(crate) const ENCODABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const ENCODABLE_SHIFT: u32 = 0x0000_0004;
	pub(crate) const HAS_GROUP_INDEX: u32 = 0x0000_0040;
	pub(crate) const GROUP_SHIFT: u32 = 0x0000_0007;
	pub(crate) const XOP_VECTOR_LENGTH_MASK: u32 = 0x0000_0003;
	pub(crate) const XOP_VECTOR_LENGTH_SHIFT: u32 = 0x0000_000A;
	pub(crate) const WBIT_MASK: u32 = 0x0000_0003;
	pub(crate) const WBIT_SHIFT: u32 = 0x0000_000C;
}
// GENERATOR-END: XopFlags

// GENERATOR-BEGIN: EvexFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EvexFlags;
impl EvexFlags {
	pub(crate) const MANDATORY_PREFIX_BYTE_MASK: u32 = 0x0000_0003;
	pub(crate) const MANDATORY_PREFIX_BYTE_SHIFT: u32 = 0x0000_0000;
	pub(crate) const EVEX_OP_CODE_TABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const EVEX_OP_CODE_TABLE_SHIFT: u32 = 0x0000_0002;
	pub(crate) const ENCODABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const ENCODABLE_SHIFT: u32 = 0x0000_0004;
	pub(crate) const HAS_GROUP_INDEX: u32 = 0x0000_0040;
	pub(crate) const GROUP_SHIFT: u32 = 0x0000_0007;
	pub(crate) const EVEX_VECTOR_LENGTH_MASK: u32 = 0x0000_0003;
	pub(crate) const EVEX_VECTOR_LENGTH_SHIFT: u32 = 0x0000_000A;
	pub(crate) const WBIT_MASK: u32 = 0x0000_0003;
	pub(crate) const WBIT_SHIFT: u32 = 0x0000_000C;
	pub(crate) const TUPLE_TYPE_MASK: u32 = 0x0000_003F;
	pub(crate) const TUPLE_TYPE_SHIFT: u32 = 0x0000_000E;
	pub(crate) const LIG: u32 = 0x0010_0000;
	pub(crate) const B: u32 = 0x0020_0000;
	pub(crate) const ER: u32 = 0x0040_0000;
	pub(crate) const SAE: u32 = 0x0080_0000;
	pub(crate) const K1: u32 = 0x0100_0000;
	pub(crate) const Z: u32 = 0x0200_0000;
}
// GENERATOR-END: EvexFlags

// GENERATOR-BEGIN: D3nowFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct D3nowFlags;
impl D3nowFlags {
	pub(crate) const ENCODABLE_MASK: u32 = 0x0000_0003;
	pub(crate) const ENCODABLE_SHIFT: u32 = 0x0000_0000;
}
// GENERATOR-END: D3nowFlags

// GENERATOR-BEGIN: OpCodeHandlerFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg(feature = "encoder")]
pub(crate) struct OpCodeHandlerFlags;
#[cfg(feature = "encoder")]
impl OpCodeHandlerFlags {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const FWAIT: u32 = 0x0000_0001;
	pub(crate) const DECLARE_DATA: u32 = 0x0000_0002;
}
// GENERATOR-END: OpCodeHandlerFlags

// GENERATOR-BEGIN: LegacyOpKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum LegacyOpKind {
	None,
	Aww,
	Adw,
	M,
	Mfbcd,
	Mf32,
	Mf64,
	Mf80,
	Mfi16,
	Mfi32,
	Mfi64,
	M14,
	M28,
	M98,
	M108,
	Mp,
	Ms,
	Mo,
	Mb,
	Mw,
	Md,
	Md_MPX,
	Mq,
	Mq_MPX,
	Mw2,
	Md2,
	Eb,
	Ew,
	Ed,
	Ed_MPX,
	Ew_d,
	Ew_q,
	Eq,
	Eq_MPX,
	Eww,
	Edw,
	Eqw,
	RdMb,
	RqMb,
	RdMw,
	RqMw,
	Gb,
	Gw,
	Gd,
	Gq,
	Rw,
	Rd,
	Rq,
	Sw,
	Cd,
	Cq,
	Dd,
	Dq,
	Td,
	Ib,
	Ib16,
	Ib32,
	Ib64,
	Iw,
	Id,
	Id64,
	Iq,
	Ib21,
	Ib11,
	Xb,
	Xw,
	Xd,
	Xq,
	Yb,
	Yw,
	Yd,
	Yq,
	wJb,
	dJb,
	qJb,
	Jw,
	wJd,
	dJd,
	qJd,
	Jxw,
	Jxd,
	Jdisp16,
	Jdisp32,
	Ob,
	Ow,
	Od,
	Oq,
	Imm1,
	B,
	BMq,
	BMo,
	MIB,
	N,
	P,
	Q,
	RX,
	VX,
	WX,
	rDI,
	MRBX,
	ES,
	CS,
	SS,
	DS,
	FS,
	GS,
	AL,
	CL,
	AX,
	DX,
	EAX,
	RAX,
	ST,
	STi,
	r8_rb,
	r16_rw,
	r32_rd,
	r64_ro,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_LEGACY_OP_KIND: [&str; 118] = [
	"None",
	"Aww",
	"Adw",
	"M",
	"Mfbcd",
	"Mf32",
	"Mf64",
	"Mf80",
	"Mfi16",
	"Mfi32",
	"Mfi64",
	"M14",
	"M28",
	"M98",
	"M108",
	"Mp",
	"Ms",
	"Mo",
	"Mb",
	"Mw",
	"Md",
	"Md_MPX",
	"Mq",
	"Mq_MPX",
	"Mw2",
	"Md2",
	"Eb",
	"Ew",
	"Ed",
	"Ed_MPX",
	"Ew_d",
	"Ew_q",
	"Eq",
	"Eq_MPX",
	"Eww",
	"Edw",
	"Eqw",
	"RdMb",
	"RqMb",
	"RdMw",
	"RqMw",
	"Gb",
	"Gw",
	"Gd",
	"Gq",
	"Rw",
	"Rd",
	"Rq",
	"Sw",
	"Cd",
	"Cq",
	"Dd",
	"Dq",
	"Td",
	"Ib",
	"Ib16",
	"Ib32",
	"Ib64",
	"Iw",
	"Id",
	"Id64",
	"Iq",
	"Ib21",
	"Ib11",
	"Xb",
	"Xw",
	"Xd",
	"Xq",
	"Yb",
	"Yw",
	"Yd",
	"Yq",
	"wJb",
	"dJb",
	"qJb",
	"Jw",
	"wJd",
	"dJd",
	"qJd",
	"Jxw",
	"Jxd",
	"Jdisp16",
	"Jdisp32",
	"Ob",
	"Ow",
	"Od",
	"Oq",
	"Imm1",
	"B",
	"BMq",
	"BMo",
	"MIB",
	"N",
	"P",
	"Q",
	"RX",
	"VX",
	"WX",
	"rDI",
	"MRBX",
	"ES",
	"CS",
	"SS",
	"DS",
	"FS",
	"GS",
	"AL",
	"CL",
	"AX",
	"DX",
	"EAX",
	"RAX",
	"ST",
	"STi",
	"r8_rb",
	"r16_rw",
	"r32_rd",
	"r64_ro",
];
impl fmt::Debug for LegacyOpKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_LEGACY_OP_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for LegacyOpKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		LegacyOpKind::None
	}
}
// GENERATOR-END: LegacyOpKind

// GENERATOR-BEGIN: VexOpKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum VexOpKind {
	None,
	Ed,
	Eq,
	Gd,
	Gq,
	RdMb,
	RqMb,
	RdMw,
	RqMw,
	Rd,
	Rq,
	Hd,
	Hq,
	HK,
	HX,
	HY,
	Ib,
	I2,
	Is4X,
	Is4Y,
	Is5X,
	Is5Y,
	M,
	Md,
	MK,
	rDI,
	RK,
	RX,
	RY,
	VK,
	VM32X,
	VM32Y,
	VM64X,
	VM64Y,
	VX,
	VY,
	WK,
	WX,
	WY,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_VEX_OP_KIND: [&str; 39] = [
	"None",
	"Ed",
	"Eq",
	"Gd",
	"Gq",
	"RdMb",
	"RqMb",
	"RdMw",
	"RqMw",
	"Rd",
	"Rq",
	"Hd",
	"Hq",
	"HK",
	"HX",
	"HY",
	"Ib",
	"I2",
	"Is4X",
	"Is4Y",
	"Is5X",
	"Is5Y",
	"M",
	"Md",
	"MK",
	"rDI",
	"RK",
	"RX",
	"RY",
	"VK",
	"VM32X",
	"VM32Y",
	"VM64X",
	"VM64Y",
	"VX",
	"VY",
	"WK",
	"WX",
	"WY",
];
impl fmt::Debug for VexOpKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_VEX_OP_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for VexOpKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		VexOpKind::None
	}
}
// GENERATOR-END: VexOpKind

// GENERATOR-BEGIN: XopOpKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum XopOpKind {
	None,
	Ed,
	Eq,
	Gd,
	Gq,
	Rd,
	Rq,
	Hd,
	Hq,
	HX,
	HY,
	Ib,
	Id,
	Is4X,
	Is4Y,
	VX,
	VY,
	WX,
	WY,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_XOP_OP_KIND: [&str; 19] = [
	"None",
	"Ed",
	"Eq",
	"Gd",
	"Gq",
	"Rd",
	"Rq",
	"Hd",
	"Hq",
	"HX",
	"HY",
	"Ib",
	"Id",
	"Is4X",
	"Is4Y",
	"VX",
	"VY",
	"WX",
	"WY",
];
impl fmt::Debug for XopOpKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_XOP_OP_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for XopOpKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		XopOpKind::None
	}
}
// GENERATOR-END: XopOpKind

// GENERATOR-BEGIN: EvexOpKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum EvexOpKind {
	None,
	Ed,
	Eq,
	Gd,
	Gq,
	RdMb,
	RqMb,
	RdMw,
	RqMw,
	HX,
	HY,
	HZ,
	HXP3,
	HZP3,
	Ib,
	M,
	Rd,
	Rq,
	RX,
	RY,
	RZ,
	RK,
	VM32X,
	VM32Y,
	VM32Z,
	VM64X,
	VM64Y,
	VM64Z,
	VK,
	VKP1,
	VX,
	VY,
	VZ,
	WX,
	WY,
	WZ,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_EVEX_OP_KIND: [&str; 36] = [
	"None",
	"Ed",
	"Eq",
	"Gd",
	"Gq",
	"RdMb",
	"RqMb",
	"RdMw",
	"RqMw",
	"HX",
	"HY",
	"HZ",
	"HXP3",
	"HZP3",
	"Ib",
	"M",
	"Rd",
	"Rq",
	"RX",
	"RY",
	"RZ",
	"RK",
	"VM32X",
	"VM32Y",
	"VM32Z",
	"VM64X",
	"VM64Y",
	"VM64Z",
	"VK",
	"VKP1",
	"VX",
	"VY",
	"VZ",
	"WX",
	"WY",
	"WZ",
];
impl fmt::Debug for EvexOpKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_EVEX_OP_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for EvexOpKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		EvexOpKind::None
	}
}
// GENERATOR-END: EvexOpKind

// GENERATOR-BEGIN: RepPrefixKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// `REP`/`REPE`/`REPNE` prefix
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
pub enum RepPrefixKind {
	/// No `REP`/`REPE`/`REPNE` prefix
	None,
	/// `REP`/`REPE` prefix
	Repe,
	/// `REPNE` prefix
	Repne,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_REP_PREFIX_KIND: [&str; 3] = [
	"None",
	"Repe",
	"Repne",
];
impl fmt::Debug for RepPrefixKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_REP_PREFIX_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for RepPrefixKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		RepPrefixKind::None
	}
}
// GENERATOR-END: RepPrefixKind
