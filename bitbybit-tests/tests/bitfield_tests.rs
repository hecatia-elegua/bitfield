use arbitrary_int::{u13, u2, u3, u30, u4, u57};
use bitbybit::bitenum;
use bitbybit::bitfield;

#[test]
fn test_construction() {
    #[bitfield(u32, default: 0)]
    struct Test2 {}

    let t = Test2::new();
    assert_eq!(0, t.raw_value);

    let t = Test2::new_with_raw_value(45);
    assert_eq!(45, t.raw_value);
}

#[test]
fn test_getter_and_setter() {
    #[bitfield(u128, default: 0)]
    struct Test2 {
        #[bits(98..=127, rw)]
        val30: u30,

        #[bits(41..=97, rw)]
        val57: u57,

        #[bits(28..=40, rw)]
        val13: u13,

        #[bits(12..=27, rw)]
        val16: u16,

        #[bits(4..=11, rw)]
        baudrate: u8,

        #[bits(0..=3, rw)]
        some_other_bits: u4,
    }

    let t = Test2::new_with_raw_value(0xAE42315A_2134FE06_3412345A_2134FE06);
    assert_eq!(u30::new(0x2B908C56), t.val30());
    assert_eq!(u57::new(0x1109A7F_031A091A), t.val57());
    assert_eq!(u13::new(0x5A2), t.val13());
    assert_eq!(0x134F, t.val16());
    assert_eq!(0xE0, t.baudrate());
    assert_eq!(u4::new(0x6), t.some_other_bits());

    let t = Test2::new()
        .with_baudrate(0x12)
        .with_some_other_bits(u4::new(0x2));
    assert_eq!(0x12, t.baudrate());
    assert_eq!(u4::new(0x2), t.some_other_bits());
    assert_eq!(0x0122, t.raw_value);
}

#[test]
fn test_getter_and_setter_arbitrary_uint() {
    #[bitfield(u128, default: 0)]
    struct Test2 {
        #[bits(4..=11, rw)]
        baudrate: u8,

        #[bits(0..=3, rw)]
        some_other_bits: u4,
    }

    let t = Test2::new_with_raw_value(0xFE06);
    assert_eq!(0xE0, t.baudrate());
    assert_eq!(u4::new(0x6), t.some_other_bits());

    let t = Test2::new()
        .with_baudrate(0x12)
        .with_some_other_bits(u4::new(0x2));
    assert_eq!(0x12, t.baudrate());
    assert_eq!(u4::new(0x2), t.some_other_bits());
    assert_eq!(0x0122, t.raw_value);
}

#[test]
fn test_bool() {
    #[bitfield(u16, default: 0)]
    struct Test {
        #[bit(0, rw)]
        bit0: bool,

        #[bit(1, rw)]
        bit1: bool,

        #[bits(2..=9, r)]
        n: u8,

        #[bits(10..=12, w)]
        m: u3,

        #[bits(13..=15, r)]
        o: u3,
    }

    let t = Test::new();
    assert!(!t.bit0());
    assert!(!t.bit1());
    assert_eq!(t.raw_value, 0b00);

    let t2 = t.with_bit0(true);
    assert!(t2.bit0());
    assert!(!t2.bit1());
    assert_eq!(t2.raw_value, 0b01);

    let t3 = t.with_bit1(true);
    assert!(!t3.bit0());
    assert!(t3.bit1());
    assert_eq!(t3.raw_value, 0b10);
}

#[test]
fn signed_vs_unsigned() {
    #[bitfield(u32)]
    struct Test {
        #[bits(0..=7, rw)]
        unsigned1: u8,

        #[bits(8..=15, rw)]
        unsigned2: u8,

        #[bits(16..=23, rw)]
        signed1: i8,

        #[bits(24..=31, rw)]
        signed2: i8,
    }

    let t = Test::new_with_raw_value(0x7FFF7FFF);
    assert_eq!(t.unsigned1(), 255);
    assert_eq!(t.unsigned2(), 127);
    assert_eq!(t.signed1(), -1);
    assert_eq!(t.signed2(), 127);

    let t2 = Test::new_with_raw_value(0)
        .with_unsigned1(0x7Fu8)
        .with_unsigned2(0xFFu8)
        .with_signed1(127)
        .with_signed2(-1);
    assert_eq!(t2.raw_value, 0xFF7FFF7F);
}

#[test]
fn default_value() {
    #[bitfield(u32, default: 0xDEADBEEF)]
    struct Test {}

    let t = Test::new();
    assert_eq!(t.raw_value, 0xDEADBEEF);
}

#[test]
fn default_value_const() {
    const DEFAULT: u32 = 0xBADCAFE;
    #[bitfield(u32, default: DEFAULT)]
    struct Test {}

    let t = Test::new();
    assert_eq!(t.raw_value, 0xBADCAFE);
}

#[test]
fn more_struct_attributes() {
    // Test that other attributes can be appended
    #[bitfield(u32)]
    #[derive(Debug)]
    struct Test {}

    // If this compiles then derive(Debug) worked
    let _ = format!("{:?}", Test::new_with_raw_value(0));
}

#[test]
fn documentation() {
    // Test that documentation can appear within the struct and doesn't trip up the parser
    #[bitfield(u32)]
    #[derive(Debug)]
    struct Test {
        /// This is documentation for a field
        #[bits(8..=15, rw)]
        field: u8,
        // A free standing comment
    }
}

#[test]
fn proper_unmasking() {
    #[bitfield(u16, default: 0)]
    pub struct TestStruct {
        #[bits(0..=1, rw)]
        a: u2,

        #[bits(2..=3, rw)]
        b: u2,

        #[bits(4..=5, rw)]
        c: u2,
    }

    let s1 = TestStruct::new()
        .with_a(u2::new(0b11))
        .with_b(u2::new(0b11))
        .with_c(u2::new(0b11));

    assert_eq!(0b111111, s1.raw_value());

    let s2 = s1.with_b(u2::new(0b00));
    assert_eq!(0b110011, s2.raw_value());
}

#[test]
fn just_one_bitrange() {
    #[bitfield(u16, default: 0)]
    pub struct JustOneBitRange {
        #[bits(0..=15, rw)]
        a: i16,
    }

    let s1 = JustOneBitRange::new().with_a(0b0111001110001111);

    assert_eq!(0b0111001110001111, s1.raw_value());
    assert_eq!(0b0111001110001111, s1.a());
}

#[test]
fn repeated_bitrange_single_bits_with_stride() {
    #[bitfield(u64, default: 0)]
    pub struct NibbleBits64 {
        #[bit(0, rw, stride: 4)]
        nibble_bit0: [bool; 16],

        #[bit(1, rw, stride: 4)]
        nibble_bit1: [bool; 16],

        #[bit(2, rw, stride: 4)]
        nibble_bit2: [bool; 16],

        #[bit(3, rw, stride: 4)]
        nibble_bit3: [bool; 16],
    }

    const VALUE: u64 = 0x12345678_ABCDEFFF;
    let nibble_bits = NibbleBits64::new_with_raw_value(VALUE);
    assert_eq!(true, nibble_bits.nibble_bit0(0));
    assert_eq!(true, nibble_bits.nibble_bit0(1));
    assert_eq!(true, nibble_bits.nibble_bit0(2));
    assert_eq!(false, nibble_bits.nibble_bit0(3));
    assert_eq!(true, nibble_bits.nibble_bit0(4));
    assert_eq!(false, nibble_bits.nibble_bit0(5));
    assert_eq!(true, nibble_bits.nibble_bit0(6));
    assert_eq!(false, nibble_bits.nibble_bit0(7));

    assert_eq!(false, nibble_bits.nibble_bit0(8));
    assert_eq!(true, nibble_bits.nibble_bit0(9));
    assert_eq!(false, nibble_bits.nibble_bit0(10));
    assert_eq!(true, nibble_bits.nibble_bit0(11));
    assert_eq!(false, nibble_bits.nibble_bit0(12));
    assert_eq!(true, nibble_bits.nibble_bit0(13));
    assert_eq!(false, nibble_bits.nibble_bit0(14));
    assert_eq!(true, nibble_bits.nibble_bit0(15));
    assert_eq!(false, nibble_bits.nibble_bit1(15));
    assert_eq!(false, nibble_bits.nibble_bit2(15));
    assert_eq!(false, nibble_bits.nibble_bit3(15));

    assert_eq!(
        0x12345678_ABCDEFFE,
        nibble_bits.with_nibble_bit0(0, false).raw_value()
    );
    assert_eq!(
        0x12345678_ABCDEFEF,
        nibble_bits.with_nibble_bit0(1, false).raw_value()
    );
    assert_eq!(
        0x12345678_ABCDEEFF,
        nibble_bits.with_nibble_bit0(2, false).raw_value()
    );
    assert_eq!(
        0x12345678_ABCDFFFF,
        nibble_bits.with_nibble_bit0(3, true).raw_value()
    );
    assert_eq!(
        0x02345678_ABCDEFFF,
        nibble_bits.with_nibble_bit0(15, false).raw_value()
    );
    assert_eq!(
        0x32345678_ABCDEFFF,
        nibble_bits.with_nibble_bit1(15, true).raw_value()
    );
    assert_eq!(
        0x52345678_ABCDEFFF,
        nibble_bits.with_nibble_bit2(15, true).raw_value()
    );
    assert_eq!(
        0x92345678_ABCDEFFF,
        nibble_bits.with_nibble_bit3(15, true).raw_value()
    );
}

#[test]
fn repeated_bitrange_single_bits_without_stride() {
    #[bitfield(u8, default: 0)]
    pub struct Bits8 {
        #[bit(0, rw)]
        bit: [bool; 8],
    }

    let bits8 = Bits8::new_with_raw_value(0b0110_1110);
    assert_eq!(false, bits8.bit(0));
    assert_eq!(true, bits8.bit(1));
    assert_eq!(true, bits8.bit(2));
    assert_eq!(true, bits8.bit(3));

    assert_eq!(false, bits8.bit(4));
    assert_eq!(true, bits8.bit(5));
    assert_eq!(true, bits8.bit(6));
    assert_eq!(false, bits8.bit(7));

    assert_eq!(0b0110_0110, bits8.with_bit(3, false).raw_value());
    assert_eq!(0b1110_1110, bits8.with_bit(7, true).raw_value());
}

#[test]
fn repeated_bitrange_without_stride() {
    #[bitfield(u64, default: 0)]
    pub struct Nibble64 {
        #[bits(0..=3, rw)]
        nibble: [u4; 16],
    }

    const VALUE: u64 = 0x12345678_ABCDEFFF;
    let nibble = Nibble64::new_with_raw_value(VALUE);

    assert_eq!(u4::new(0xF), nibble.nibble(0));
    assert_eq!(u4::new(0xF), nibble.nibble(1));
    assert_eq!(u4::new(0xF), nibble.nibble(2));
    assert_eq!(u4::new(0xE), nibble.nibble(3));
    assert_eq!(u4::new(0xD), nibble.nibble(4));
    assert_eq!(u4::new(0xC), nibble.nibble(5));

    assert_eq!(
        0x12345678_ABCDEFF3,
        nibble.with_nibble(0, u4::new(0x3)).raw_value()
    );
    assert_eq!(
        0x12345678_ABCDEF2F,
        nibble.with_nibble(1, u4::new(0x2)).raw_value()
    );
    assert_eq!(
        0x12345678_ABCDEAFF,
        nibble.with_nibble(2, u4::new(0xA)).raw_value()
    );
    assert_eq!(
        0xE2345678_ABCDEFFF,
        nibble.with_nibble(15, u4::new(0xE)).raw_value()
    );
}

#[test]
fn repeated_bitrange_with_stride_equals_width() {
    #[bitfield(u64, default: 0)]
    pub struct Nibble64 {
        #[bits(0..=3, rw, stride: 4)]
        nibble: [u4; 16],
    }

    const VALUE: u64 = 0x12345678_ABCDEFFF;
    let nibble = Nibble64::new_with_raw_value(VALUE);

    assert_eq!(u4::new(0xF), nibble.nibble(0));
    assert_eq!(u4::new(0xF), nibble.nibble(1));
    assert_eq!(u4::new(0xF), nibble.nibble(2));
    assert_eq!(u4::new(0xE), nibble.nibble(3));
    assert_eq!(u4::new(0xD), nibble.nibble(4));
    assert_eq!(u4::new(0xC), nibble.nibble(5));

    assert_eq!(
        0x12345678_ABCDEFF3,
        nibble.with_nibble(0, u4::new(0x3)).raw_value()
    );
    assert_eq!(
        0x12345678_ABCDEF2F,
        nibble.with_nibble(1, u4::new(0x2)).raw_value()
    );
    assert_eq!(
        0x12345678_ABCDEAFF,
        nibble.with_nibble(2, u4::new(0xA)).raw_value()
    );
    assert_eq!(
        0xE2345678_ABCDEFFF,
        nibble.with_nibble(15, u4::new(0xE)).raw_value()
    );
}

#[test]
fn repeated_bitrange_with_stride_greater_than_width() {
    #[bitfield(u64, default: 0)]
    pub struct EvenNibble64 {
        #[bits(0..=3, rw, stride: 8)]
        even_nibble: [u4; 8],
    }

    const VALUE: u64 = 0x12345678_ABCDEFFF;
    let even_nibble = EvenNibble64::new_with_raw_value(VALUE);

    assert_eq!(u4::new(0xF), even_nibble.even_nibble(0));
    assert_eq!(u4::new(0xF), even_nibble.even_nibble(1));
    assert_eq!(u4::new(0xD), even_nibble.even_nibble(2));
    assert_eq!(u4::new(0xB), even_nibble.even_nibble(3));
    assert_eq!(u4::new(0x8), even_nibble.even_nibble(4));
    assert_eq!(u4::new(0x6), even_nibble.even_nibble(5));
}

#[test]
fn repeated_bitrange_with_stride_greater_than_width_basic_type() {
    #[bitfield(u64, default: 0)]
    pub struct EvenByte64 {
        #[bits(0..=7, rw, stride: 16)]
        even_byte: [u8; 4],
    }

    const VALUE: u64 = 0x12345678_ABCDEFFF;
    let even_byte = EvenByte64::new_with_raw_value(VALUE);

    assert_eq!(0xFF, even_byte.even_byte(0));
    assert_eq!(0xCD, even_byte.even_byte(1));
    assert_eq!(0x78, even_byte.even_byte(2));
    assert_eq!(0x34, even_byte.even_byte(3));
}

#[test]
fn bitfield_with_enum_exhaustive() {
    #[bitenum(u2, exhaustive: true)]
    #[derive(Eq, PartialEq, Debug)]
    pub enum ExhaustiveEnum {
        Zero = 0b00,
        One = 0b01,
        Two = 0b10,
        Three = 0b11,
    }

    #[bitfield(u64, default: 0)]
    pub struct BitfieldWithEnum {
        #[bits(0..=1, rw)]
        e1: ExhaustiveEnum,
    }

    assert_eq!(
        ExhaustiveEnum::Zero,
        BitfieldWithEnum::new_with_raw_value(0b1100).e1()
    );
    assert_eq!(
        ExhaustiveEnum::One,
        BitfieldWithEnum::new_with_raw_value(0b1101).e1()
    );
    assert_eq!(
        ExhaustiveEnum::Two,
        BitfieldWithEnum::new_with_raw_value(0b1110).e1()
    );
    assert_eq!(
        ExhaustiveEnum::Three,
        BitfieldWithEnum::new_with_raw_value(0b1111).e1()
    );

    assert_eq!(
        0b10,
        BitfieldWithEnum::new()
            .with_e1(ExhaustiveEnum::Two)
            .raw_value()
    );
    assert_eq!(
        0b11,
        BitfieldWithEnum::new()
            .with_e1(ExhaustiveEnum::Three)
            .raw_value()
    );
}

#[test]
fn bitfield_with_enum_nonexhaustive() {
    #[bitenum(u2, exhaustive: false)]
    #[derive(Eq, PartialEq, Debug)]
    pub enum NonExhaustiveEnum {
        Zero = 0b00,
        One = 0b01,
        Two = 0b10,
    }

    #[bitfield(u64, default: 0)]
    pub struct BitfieldWithEnumNonExhaustive {
        #[bits(2..=3, rw)]
        e2: Option<NonExhaustiveEnum>,
    }

    assert_eq!(
        Ok(NonExhaustiveEnum::Zero),
        BitfieldWithEnumNonExhaustive::new_with_raw_value(0b0010).e2()
    );
    assert_eq!(
        Ok(NonExhaustiveEnum::One),
        BitfieldWithEnumNonExhaustive::new_with_raw_value(0b0110).e2()
    );
    assert_eq!(
        Ok(NonExhaustiveEnum::Two),
        BitfieldWithEnumNonExhaustive::new_with_raw_value(0b1010).e2()
    );
    assert_eq!(
        Err(3),
        BitfieldWithEnumNonExhaustive::new_with_raw_value(0b1110).e2()
    );

    assert_eq!(
        0b0000,
        BitfieldWithEnumNonExhaustive::new()
            .with_e2(NonExhaustiveEnum::Zero)
            .raw_value()
    );
    assert_eq!(
        0b0100,
        BitfieldWithEnumNonExhaustive::new()
            .with_e2(NonExhaustiveEnum::One)
            .raw_value()
    );
    assert_eq!(
        0b1000,
        BitfieldWithEnumNonExhaustive::new()
            .with_e2(NonExhaustiveEnum::Two)
            .raw_value()
    );
    // No way to set to Three (by design): If an enum member doesn't exist, we shouldn't be
    // able to write it
}

#[test]
fn bitfield_with_indexed_exhaustive_enum() {
    #[bitenum(u2, exhaustive: true)]
    #[derive(Eq, PartialEq, Debug)]
    pub enum ExhaustiveEnum {
        Zero = 0b00,
        One = 0b01,
        Two = 0b10,
        Three = 0b11,
    }

    #[bitfield(u64, default: 0)]
    pub struct BitfieldWithIndexedEnums {
        #[bits(0..=1, rw)]
        exhaustive: [ExhaustiveEnum; 8],
    }

    assert_eq!(
        ExhaustiveEnum::Two,
        BitfieldWithIndexedEnums::new_with_raw_value(0b0010).exhaustive(0)
    );
    assert_eq!(
        ExhaustiveEnum::One,
        BitfieldWithIndexedEnums::new_with_raw_value(0b0110).exhaustive(1)
    );
    assert_eq!(
        ExhaustiveEnum::Zero,
        BitfieldWithIndexedEnums::new_with_raw_value(0b0110).exhaustive(2)
    );

    assert_eq!(
        0b11_01_10,
        BitfieldWithIndexedEnums::new_with_raw_value(0b01_10)
            .with_exhaustive(2, ExhaustiveEnum::Three)
            .raw_value()
    );
}

#[test]
fn bitfield_with_indexed_nonexhaustive_enum() {
    #[bitenum(u2, exhaustive: false)]
    #[derive(Eq, PartialEq, Debug)]
    pub enum NonExhaustiveEnum {
        Zero = 0b00,
        One = 0b01,
        Two = 0b10,
    }

    #[bitfield(u64, default: 0)]
    pub struct BitfieldWithIndexedEnums {
        #[bits(0..=1, rw)]
        nonexhaustive: [Option<NonExhaustiveEnum>; 8],
    }

    assert_eq!(
        Ok(NonExhaustiveEnum::Two),
        BitfieldWithIndexedEnums::new_with_raw_value(0b0010).nonexhaustive(0)
    );
    assert_eq!(
        Ok(NonExhaustiveEnum::One),
        BitfieldWithIndexedEnums::new_with_raw_value(0b0110).nonexhaustive(1)
    );
    assert_eq!(
        Ok(NonExhaustiveEnum::Zero),
        BitfieldWithIndexedEnums::new_with_raw_value(0b0110).nonexhaustive(2)
    );

    assert_eq!(
        0b10_01_10,
        BitfieldWithIndexedEnums::new_with_raw_value(0b01_10)
            .with_nonexhaustive(2, NonExhaustiveEnum::Two)
            .raw_value()
    );
}

#[test]
fn bitfield_with_u8_enum() {
    #[bitenum(u8, exhaustive: false)]
    #[derive(Eq, PartialEq, Debug)]
    pub enum NonExhaustiveEnum {
        Zero = 0b00,
        One = 0b01,
        Two = 0b10000010,
    }

    #[bitfield(u64, default: 0)]
    pub struct BitfieldWithIndexedEnums {
        #[bits(6..=13, rw)]
        val8: Option<NonExhaustiveEnum>,
    }

    assert_eq!(
        Ok(NonExhaustiveEnum::Zero),
        BitfieldWithIndexedEnums::new_with_raw_value(0b00_00000000_000000).val8()
    );
    assert_eq!(
        Ok(NonExhaustiveEnum::One),
        BitfieldWithIndexedEnums::new_with_raw_value(0b00_00000001_000000).val8()
    );
    assert_eq!(
        Ok(NonExhaustiveEnum::Two),
        BitfieldWithIndexedEnums::new_with_raw_value(0b00_10000010_000000).val8()
    );

    assert_eq!(
        Err(0b00100000),
        BitfieldWithIndexedEnums::new_with_raw_value(0b00_00100000_000000).val8()
    );
}
