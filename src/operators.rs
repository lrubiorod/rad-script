
use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum RadonOpCodes {
    /// Only for the sake of allowing catch-alls when matching
    Fail = 0xFF,
    ///////////////////////////////////////////////////////////////////////
    // Multi-type operator codes start at 0x00
    Identity = 0x00,
    ///////////////////////////////////////////////////////////////////////
    // Array operator codes (start at 0x10)
    ArrayCount = 0x10,
    ArrayFilter = 0x11,
    //    ArrayFlatten = 0x12,
    ArrayGetArray = 0x13,
    ArrayGetBoolean = 0x14,
    ArrayGetBytes = 0x15,
    ArrayGetFloat = 0x16,
    ArrayGetInteger = 0x17,
    ArrayGetMap = 0x18,
    ArrayGetString = 0x19,
    ArrayMap = 0x1A,
    ArrayReduce = 0x1B,
    //    ArraySome = 0x1C,
    ArraySort = 0x1D,
    //    ArrayTake = 0x1E,
    ///////////////////////////////////////////////////////////////////////
    // Boolean operator codes (start at 0x20)
    BooleanAsString = 0x20,
    //    BooleanMatch = 0x21,
    BooleanNegate = 0x22,
    ///////////////////////////////////////////////////////////////////////
    // Bytes operator codes (start at 0x30)
    BytesAsString = 0x30,
    BytesHash = 0x31,
    ///////////////////////////////////////////////////////////////////////
    // Integer operator codes (start at 0x40)
    IntegerAbsolute = 0x40,
    IntegerAsFloat = 0x41,
    IntegerAsString = 0x42,
    IntegerGreaterThan = 0x43,
    IntegerLessThan = 0x44,
    //    IntegerMatch = 0x45,
    IntegerModulo = 0x46,
    IntegerMultiply = 0x47,
    IntegerNegate = 0x48,
    IntegerPower = 0x49,
    //    IntegerReciprocal = 0x4A,
    //    IntegerSum = 0x4B,
    ///////////////////////////////////////////////////////////////////////
    // Float operator codes (start at 0x50)
    FloatAbsolute = 0x50,
    FloatAsString = 0x51,
    FloatCeiling = 0x52,
    FloatGreaterThan = 0x53,
    FloatFloor = 0x54,
    FloatLessThan = 0x55,
    FloatModulo = 0x56,
    FloatMultiply = 0x57,
    FloatNegate = 0x58,
    FloatPower = 0x59,
    //    FloatReciprocal = 0x5A,
    FloatRound = 0x5B,
    //    FloatSum = 0x5C,
    FloatTruncate = 0x5D,
    ///////////////////////////////////////////////////////////////////////
    // Map operator codes (start at 0x60)
    //    MapEntries = 0x60,
    MapGetArray = 0x61,
    MapGetBoolean = 0x62,
    MapGetBytes = 0x63,
    MapGetFloat = 0x64,
    MapGetInteger = 0x65,
    MapGetMap = 0x66,
    MapGetString = 0x67,
    MapKeys = 0x68,
    MapValues = 0x69,
    ///////////////////////////////////////////////////////////////////////
    // String operator codes (start at 0x70)
    StringAsBoolean = 0x70,
    //    StringAsBytes = 0x71,
    StringAsFloat = 0x72,
    StringAsInteger = 0x73,
    StringLength = 0x74,
    StringMatch = 0x75,
    StringParseJSONArray = 0x76,
    StringParseJSONMap = 0x77,
    //    StringParseXML = 0x78,
    StringToLowerCase = 0x79,
    StringToUpperCase = 0x7A,
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum RadonReducers {
    Min = 0x00,
    Max = 0x01,
    Mode = 0x02,
    AverageMean = 0x03,
    AverageMeanWeighted = 0x04,
    AverageMedian = 0x05,
    AverageMedianWeighted = 0x06,
    DeviationStandard = 0x07,
    DeviationAverageAbsolute = 0x08,
    DeviationMedianAbsolute = 0x09,
    DeviationMaximumAbsolute = 0x10,
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum RadonFilters {
    GreaterThan = 0x00,
    LessThan = 0x01,
    Equals = 0x02,
    DeviationAbsolute = 0x03,
    DeviationRelative = 0x04,
    DeviationStandard = 0x05,
    Top = 0x06,
    Bottom = 0x07,
    LessOrEqualThan = 0x80,
    GreaterOrEqualThan = 0x81,
    NotEquals = 0x82,
    NotDeviationAbsolute = 0x83,
    NotDeviationRelative = 0x84,
    NotDeviationStandard = 0x85,
    NotTop = 0x86,
    NotBottom = 0x87,
}
