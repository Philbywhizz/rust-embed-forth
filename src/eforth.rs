pub const EFORTH_CORE_SIZE: usize = 2751;
/// `EFORTH_CORE` has been generated from `eforth.blk`, it contains a full eForth
/// like interpreter executable by the embed virtual machine
pub const EFORTH_CORE: [u16; EFORTH_CORE_SIZE] = [
	0x0952, 0x034d, 0x4689, 0x4854, 0x0a0d, 0x0a1a, 0x157e, 0x3437, 0x0001, 0x1984, 0x0001, 0x628d, 0x601c, 0x628d, 0x631c, 0x157e,
	0x10dc, 0x1574, 0x14ac, 0x0024, 0x0000, 0x6403, 0x7075, 0x609d, 0x0028, 0x6f04, 0x6576, 0x0072, 0x619d, 0x0030, 0x6906, 0x766e,
	0x7265, 0x0074, 0x6a1c, 0x003a, 0x7503, 0x2b6d, 0x651c, 0x0046, 0x2b01, 0x653f, 0x004e, 0x7503, 0x2a6d, 0x661c, 0x0054, 0x2a01,
	0x663f, 0x005c, 0x7304, 0x6177, 0x0070, 0x619c, 0x0062, 0x6e03, 0x7069, 0x601f, 0x006c, 0x6404, 0x6f72, 0x0070, 0x611f, 0x0074,
	0x4001, 0x631c, 0x007e, 0x2101, 0x641f, 0x0084, 0x7206, 0x6873, 0x6669, 0x0074, 0x701f, 0x008a, 0x6c06, 0x6873, 0x6669, 0x0074,
	0x711f, 0x0096, 0x3d01, 0x6d1f, 0x00a2, 0x7502, 0x003c, 0x6e1f, 0x00a8, 0x3c01, 0x6f1f, 0x00b0, 0x6103, 0x646e, 0x671f, 0x00b6,
	0x7803, 0x726f, 0x691f, 0x00be, 0x6f02, 0x0072, 0x681f, 0x00c6, 0x3102, 0x002d, 0x6b1c, 0x00ce, 0x3002, 0x003d, 0x6c1c, 0x00d6,
	0x2805, 0x7962, 0x2965, 0x7b1c, 0x00de, 0x7203, 0x3f78, 0x789d, 0x00e8, 0x7403, 0x2178, 0x773f, 0x00f0, 0x2806, 0x6173, 0x6576,
	0x0029, 0x761f, 0x00f8, 0x7505, 0x6d2f, 0x646f, 0x799c, 0x0104, 0x2f04, 0x6f6d, 0x0064, 0x7a9c, 0x010e, 0x2f01, 0x7a1f, 0x0118,
	0x6d03, 0x646f, 0x7a3f, 0x811e, 0x6504, 0x6978, 0x0074, 0x601c, 0x8126, 0x3e02, 0x0072, 0x6147, 0x8130, 0x7202, 0x003e, 0x628d,
	0x8138, 0x7202, 0x0040, 0x6281, 0x8140, 0x7205, 0x7264, 0x706f, 0x600c, 0x0148, 0x6304, 0x6c65, 0x006c, 0x400d, 0x0002, 0x0152,
	0x3e03, 0x6e69, 0x400b, 0x0000, 0x015e, 0x7305, 0x6174, 0x6574, 0x400b, 0x0000, 0x0168, 0x6803, 0x646c, 0x400b, 0x0000, 0x0174,
	0x6204, 0x7361, 0x0065, 0x400b, 0x0010, 0x017e, 0x7304, 0x6170, 0x006e, 0x400b, 0x0000, 0x018a, 0x2305, 0x6f76, 0x7363, 0x400d,
	0x0008, 0x0196, 0x6205, 0x622f, 0x6675, 0x400d, 0x0400, 0x01a2, 0x6203, 0x6b6c, 0x400b, 0x0000, 0x01ae, 0x7003, 0x6461, 0x400d,
	0x4280, 0x01b8, 0x3c09, 0x696c, 0x6574, 0x6172, 0x3e6c, 0x400b, 0x0c32, 0x01c2, 0x3c06, 0x6f62, 0x746f, 0x003e, 0x400b, 0x12ec,
	0x01d2, 0x3c04, 0x6b6f, 0x003e, 0x400b, 0x0000, 0x8000, 0x6a1c, 0xffff, 0x6a1c, 0x6103, 0x6103, 0x8000, 0x601c, 0x8172, 0x631c,
	0x8001, 0x671f, 0x8166, 0x641f, 0x8166, 0x631c, 0x01e0, 0x3205, 0x7264, 0x706f, 0x6103, 0x611f, 0x020c, 0x3102, 0x002b, 0x8001,
	0x653f, 0x0218, 0x6e06, 0x6765, 0x7461, 0x0065, 0x6a00, 0x010f, 0x0222, 0x2d01, 0x4116, 0x653f, 0x6181, 0x011a, 0x6181, 0x653f,
	0x0230, 0x6107, 0x696c, 0x6e67, 0x6465, 0x6081, 0x4100, 0x653f, 0x0240, 0x6203, 0x6579, 0x8000, 0x7b1c, 0x8002, 0x011a, 0x0250,
	0x6305, 0x6c65, 0x2b6c, 0x8002, 0x653f, 0x025e, 0x6305, 0x6c65, 0x736c, 0x8001, 0x711f, 0x026a, 0x6305, 0x6168, 0x7372, 0x8001,
	0x701f, 0x0276, 0x3f04, 0x7564, 0x0070, 0x6081, 0x2148, 0x609d, 0x601c, 0x0282, 0x3e01, 0x6180, 0x6f1f, 0x0292, 0x7502, 0x003e,
	0x6180, 0x6e1f, 0x6e03, 0x6a1c, 0x029a, 0x3c02, 0x003e, 0x6d03, 0x6a1c, 0x02a8, 0x3003, 0x3e3c, 0x6c00, 0x6a1c, 0x02b2, 0x3002,
	0x003e, 0x8000, 0x014b, 0x02bc, 0x3002, 0x003c, 0x8000, 0x6f1f, 0x02c6, 0x3204, 0x7564, 0x0070, 0x6181, 0x619d, 0x02d0, 0x7404,
	0x6375, 0x006b, 0x6180, 0x619d, 0x02dc, 0x2b02, 0x0021, 0x4172, 0x6300, 0x6523, 0x6180, 0x641f, 0x02e8, 0x3103, 0x212b, 0x8001,
	0x6180, 0x0177, 0x02f8, 0x3103, 0x212d, 0x40f6, 0x6180, 0x0177, 0x0304, 0x3202, 0x0021, 0x4172, 0x6403, 0x4133, 0x641f, 0x0310,
	0x3202, 0x0040, 0x6081, 0x4133, 0x6300, 0x6180, 0x631c, 0x031e, 0x670b, 0x7465, 0x632d, 0x7275, 0x6572, 0x746e, 0x8026, 0x631c,
	0x032e, 0x730b, 0x7465, 0x632d, 0x7275, 0x6572, 0x746e, 0x8026, 0x641f, 0x0340, 0x6202, 0x006c, 0x8020, 0x601c, 0x0352, 0x7706,
	0x7469, 0x6968, 0x006e, 0x411c, 0x6147, 0x411a, 0x628d, 0x6e1f, 0x035c, 0x6103, 0x7362, 0x6081, 0x4166, 0x21bf, 0x0116, 0x601c,
	0x0370, 0x7403, 0x6269, 0xc122, 0x4133, 0x631c, 0x0380, 0x7306, 0x756f, 0x6372, 0x0065, 0xc122, 0x0192, 0x038c, 0x7309, 0x756f,
	0x6372, 0x2d65, 0x6469, 0xc006, 0x631c, 0x039a, 0x6403, 0x3d30, 0x6c00, 0x6180, 0x6c00, 0x671f, 0x03aa, 0x6407, 0x656e, 0x6167,
	0x6574, 0x6a00, 0x6147, 0x6a00, 0x8001, 0x6500, 0x628d, 0x653f, 0x03b8, 0x6507, 0x6578, 0x7563, 0x6574, 0x6147, 0x601c, 0x6300,
	0x4145, 0x21f3, 0x6147, 0x601c, 0x03d0, 0x6302, 0x0040, 0x6381, 0x6180, 0x4100, 0x21fd, 0x8008, 0x701f, 0x80ff, 0x671f, 0x03e8,
	0x6302, 0x0021, 0x6180, 0x80ff, 0x6703, 0x6081, 0x8008, 0x7103, 0x6803, 0x6180, 0x6180, 0x6181, 0x6081, 0x6300, 0x6180, 0x4100,
	0x6c00, 0x80ff, 0x6903, 0x6147, 0x6181, 0x6903, 0x628d, 0x6703, 0x6903, 0x6180, 0x641f, 0x40fe, 0x6c1c, 0x03fe, 0x6804, 0x7265,
	0x0065, 0x801e, 0x631c, 0x043a, 0x6105, 0x696c, 0x6e67, 0x4221, 0x4125, 0x801e, 0x641f, 0x0446, 0x6105, 0x6c6c, 0x746f, 0x801e,
	0x0177, 0x0456, 0x7203, 0x746f, 0x6147, 0x6180, 0x628d, 0x619c, 0x0462, 0x2d04, 0x6f72, 0x0074, 0x6180, 0x6147, 0x6180, 0x628d,
	0x601c, 0x6240, 0x6180, 0x6147, 0x6147, 0x601c, 0x628d, 0x628d, 0x6180, 0x6240, 0x601c, 0x4246, 0x4145, 0x2253, 0x6b00, 0x6147,
	0x6300, 0x6147, 0x601c, 0x4133, 0x6147, 0x601c, 0x0470, 0x6d03, 0x6e69, 0x416c, 0x6f03, 0x225d, 0x611f, 0x601f, 0x04ac, 0x6d03,
	0x7861, 0x416c, 0x414b, 0x025b, 0x04bc, 0x6b03, 0x7965, 0xc010, 0x41ef, 0x6081, 0x40f6, 0x6d03, 0x226f, 0x412b, 0x00fc, 0x601c,
	0x04c8, 0x2f07, 0x7473, 0x6972, 0x676e, 0x6181, 0x4259, 0x4234, 0x411e, 0x423c, 0x011a, 0x8001, 0x0275, 0x04e0, 0x6305, 0x756f,
	0x746e, 0x6081, 0x410f, 0x6180, 0x01f7, 0x6181, 0x01f7, 0x6181, 0x8008, 0x7003, 0x6903, 0x6081, 0x8004, 0x7003, 0x6903, 0x6081,
	0x8005, 0x7103, 0x6903, 0x6081, 0x800c, 0x7103, 0x6903, 0x6180, 0x8008, 0x7103, 0x691f, 0x04fa, 0x6303, 0x6372, 0x8000, 0x6a00,
	0x6147, 0x6081, 0x22ab, 0x4285, 0x628d, 0x6180, 0x4287, 0x6147, 0x8001, 0x4275, 0x02a1, 0x410a, 0x628d, 0x601c, 0x6300, 0xbfff,
	0x671f, 0x419e, 0x02ae, 0x0536, 0x6504, 0x696d, 0x0074, 0xc012, 0x01ef, 0x0566, 0x6302, 0x0072, 0x800d, 0x42b7, 0x800a, 0x02b7,
	0x803a, 0x02b7, 0x0572, 0x7305, 0x6170, 0x6563, 0x8020, 0x02b7, 0x8020, 0x6180, 0x8000, 0x4261, 0x6147, 0x02d0, 0x6081, 0x42b7,
	0x424b, 0x059c, 0x611f, 0x0584, 0x6405, 0x7065, 0x6874, 0x7281, 0xc400, 0x411a, 0x013f, 0x05a6, 0x7004, 0x6369, 0x006b, 0x4139,
	0x7281, 0x6180, 0x411a, 0x631c, 0x807f, 0x6703, 0x6081, 0x807f, 0x8020, 0x41b3, 0x22ed, 0x6103, 0x805f, 0x601c, 0x05b6, 0x7404,
	0x7079, 0x0065, 0x8000, 0x6147, 0x6081, 0x22ff, 0x6180, 0x4281, 0x6281, 0x22fb, 0x42e4, 0x42b7, 0x6180, 0x6b00, 0x02f4, 0x600c,
	0x010a, 0x4281, 0x02f2, 0x40f6, 0x02f3, 0x05dc, 0x6305, 0x6f6d, 0x6576, 0x6147, 0x0313, 0x6147, 0x6081, 0x41f7, 0x6281, 0x4202,
	0x410f, 0x628d, 0x410f, 0x424b, 0x0616, 0x010a, 0x060a, 0x6604, 0x6c69, 0x006c, 0x6180, 0x6147, 0x6180, 0x0321, 0x416c, 0x4202,
	0x410f, 0x424b, 0x063c, 0x010a, 0x6147, 0x0327, 0x6103, 0x424b, 0x064c, 0x601c, 0x062c, 0x6305, 0x7461, 0x6863, 0x7281, 0x6147,
	0xc00a, 0x6300, 0x6147, 0x7381, 0xc00a, 0x6403, 0x41ed, 0x628d, 0xc00a, 0x6403, 0x628d, 0x00fb, 0x0654, 0x7405, 0x7268, 0x776f,
	0x4145, 0x234c, 0xc00a, 0x6300, 0x7503, 0x628d, 0xc00a, 0x6403, 0x6240, 0x7400, 0x6103, 0x628d, 0x601c, 0x4116, 0x0340, 0x8001,
	0x42d7, 0x6b00, 0x4150, 0x2356, 0x8004, 0x034d, 0x601c, 0x8002, 0x0350, 0x0678, 0x7506, 0x2f6d, 0x6f6d, 0x0064, 0x4145, 0x6c00,
	0x2363, 0x800a, 0x034d, 0x416c, 0x6e03, 0x2386, 0x4116, 0x800f, 0x6147, 0x6147, 0x6081, 0x6500, 0x6147, 0x6147, 0x6081, 0x6500,
	0x628d, 0x6523, 0x6081, 0x628d, 0x6281, 0x6180, 0x6147, 0x6500, 0x628d, 0x6803, 0x2380, 0x6147, 0x6103, 0x410f, 0x628d, 0x0381,
	0x6103, 0x628d, 0x424b, 0x06d2, 0x6103, 0x619c, 0x6103, 0x410a, 0x40f6, 0x609d, 0x06b2, 0x6407, 0x6365, 0x6d69, 0x6c61, 0x800a,
	0x8188, 0x641f, 0x0714, 0x6803, 0x7865, 0x8010, 0x8188, 0x641f, 0x8188, 0x6300, 0x6081, 0x8002, 0x411a, 0x8022, 0x4150, 0x23a3,
	0x4395, 0x8028, 0x034d, 0x601c, 0x0724, 0x6804, 0x6c6f, 0x0064, 0x817c, 0x6300, 0x6b00, 0x6081, 0x817c, 0x6403, 0x4202, 0x817c,
	0x6300, 0xc280, 0x8100, 0x6523, 0x4150, 0x23b8, 0x8011, 0x034d, 0x601c, 0x6081, 0x6147, 0x435e, 0x628d, 0x6180, 0x6147, 0x435e,
	0x628d, 0x0234, 0x8009, 0x6181, 0x6f03, 0x8007, 0x6703, 0x6523, 0x8030, 0x653f, 0x0748, 0x2302, 0x003e, 0x410a, 0x817c, 0x6300,
	0xc280, 0x6181, 0x011a, 0x0794, 0x2301, 0x4357, 0x8000, 0x8188, 0x6300, 0x43b9, 0x43c2, 0x03a8, 0x07a6, 0x2302, 0x0073, 0x43d5,
	0x416c, 0x41d8, 0x23df, 0x601c, 0x07b8, 0x3c02, 0x0023, 0xc280, 0x817c, 0x641f, 0x07c8, 0x7304, 0x6769, 0x006e, 0x4166, 0x23f2,
	0x802d, 0x03a8, 0x601c, 0x6044, 0x41bb, 0x8000, 0x43e7, 0x43df, 0x628d, 0x43ee, 0x03cd, 0x8000, 0x43e7, 0x43df, 0x03cd, 0x07d4,
	0x7503, 0x722e, 0x6147, 0x43fb, 0x628d, 0x411c, 0x42c8, 0x02f2, 0x8005, 0x0402, 0x07fe, 0x7502, 0x002e, 0x43fb, 0x42c6, 0x02f2,
	0x0814, 0x2e01, 0x4398, 0x800a, 0x6903, 0x2417, 0x040d, 0x43f3, 0x42c6, 0x02f2, 0xc000, 0x4221, 0x011a, 0x441a, 0x040d, 0x0820,
	0x7005, 0x6361, 0x246b, 0x4125, 0x6044, 0x6181, 0x6081, 0x8002, 0x4116, 0x6703, 0x411a, 0x411e, 0x8000, 0x417a, 0x416c, 0x4202,
	0x410f, 0x6180, 0x4309, 0x628d, 0x601c, 0x083e, 0x3d07, 0x7473, 0x6972, 0x676e, 0x6147, 0x6180, 0x628d, 0x6181, 0x6903, 0x2442,
	0x6103, 0x00fa, 0x6147, 0x044d, 0x4281, 0x6147, 0x6180, 0x4281, 0x628d, 0x6903, 0x244d, 0x600c, 0x00fa, 0x424b, 0x0888, 0x410a,
	0x00f6, 0x6181, 0x4202, 0x010f, 0x086a, 0x6106, 0x6363, 0x7065, 0x0074, 0x411e, 0x6181, 0x6981, 0x2468, 0x4267, 0x6081, 0x800a,
	0x6903, 0x2464, 0x4451, 0x0467, 0x6103, 0x6003, 0x6081, 0x045b, 0x6103, 0x011c, 0x08a8, 0x6506, 0x7078, 0x6365, 0x0074, 0xc014,
	0x41ef, 0x8194, 0x6403, 0x611f, 0x08d4, 0x7105, 0x6575, 0x7972, 0x41c3, 0x8050, 0xc014, 0x41ef, 0xc122, 0x6403, 0x40fb, 0x0102,
	0x08e8, 0x6e03, 0x6166, 0x42af, 0x0133, 0x0900, 0x6303, 0x6166, 0x4483, 0x6081, 0x41f7, 0x6523, 0x4133, 0x8001, 0x6a00, 0x671f,
	0x4483, 0x0301, 0x6300, 0xc000, 0x6703, 0x6c00, 0x6c1c, 0x6300, 0x40f8, 0x6703, 0x0495, 0x8126, 0x8152, 0x01b3, 0x6180, 0x6147,
	0x6081, 0x6081, 0x24b6, 0x6081, 0x4483, 0x4281, 0x6281, 0x4281, 0x443a, 0x24b2, 0x6081, 0x4492, 0x24af, 0x8001, 0x04b0, 0x40f6,
	0x600c, 0x601c, 0x6003, 0x6081, 0x42ae, 0x04a1, 0x600c, 0x00fa, 0x6147, 0xc110, 0x6381, 0x24cb, 0x6381, 0x6300, 0x6281, 0x6180,
	0x449e, 0x4145, 0x24c9, 0x6147, 0x4234, 0x6103, 0x628d, 0x600c, 0x601c, 0x4133, 0x04ba, 0x40fb, 0x628d, 0x00fc, 0x090a, 0x730f,
	0x6165, 0x6372, 0x2d68, 0x6f77, 0x6472, 0x696c, 0x7473, 0x449e, 0x4234, 0x611f, 0x099c, 0x6604, 0x6e69, 0x0064, 0x44b8, 0x4234,
	0x611f, 0x8030, 0x803a, 0x01b3, 0x8061, 0x807b, 0x01b3, 0x8041, 0x805b, 0x01b3, 0x6081, 0x44e7, 0x24ef, 0x8020, 0x691f, 0x601c,
	0x44ea, 0x6081, 0x44e4, 0x24f6, 0x8057, 0x011a, 0x6081, 0x44e1, 0x24fb, 0x8030, 0x011a, 0x6103, 0x00f6, 0x44ea, 0x44f0, 0x8188,
	0x6300, 0x6e1f, 0x416c, 0x4241, 0x6103, 0x41f7, 0x6081, 0x44fd, 0x2511, 0x6180, 0x8188, 0x6300, 0x6623, 0x6180, 0x44f0, 0x6523,
	0x0514, 0x6103, 0x4246, 0x601c, 0x4246, 0x427b, 0x6081, 0x6c00, 0x2502, 0x601c, 0x4285, 0x802d, 0x6d03, 0x2520, 0x427b, 0x00f6,
	0x00fc, 0x4285, 0x8024, 0x6d03, 0x2527, 0x427b, 0x0395, 0x4285, 0x8023, 0x6d03, 0x252d, 0x427b, 0x038f, 0x601c, 0x09b4, 0x3e07,
	0x756e, 0x626d, 0x7265, 0x4398, 0x6147, 0x451a, 0x6147, 0x4521, 0x4502, 0x628d, 0x253e, 0x4234, 0x4116, 0x423c, 0x628d, 0x8188,
	0x641f, 0x8000, 0x423c, 0x4533, 0x6003, 0x6c1c, 0x6147, 0x0551, 0x8020, 0x6181, 0x6281, 0x6523, 0x41f7, 0x6f03, 0x2551, 0x628d,
	0x010f, 0x424b, 0x0a90, 0x00fc, 0x6147, 0x6081, 0x2564, 0x4285, 0x6281, 0x411a, 0x6281, 0x8020, 0x6d03, 0xc000, 0x41ef, 0x2562,
	0x600c, 0x601c, 0x427b, 0x0555, 0x600c, 0x601c, 0x2568, 0x0161, 0x015c, 0x4566, 0x6a1c, 0x8acc, 0xc000, 0x6403, 0x0554, 0x8ad2,
	0xc000, 0x6403, 0x0554, 0x6147, 0x6181, 0x628d, 0x6180, 0x4241, 0x6281, 0x456b, 0x416c, 0x628d, 0x456f, 0x6180, 0x628d, 0x411a,
	0x6147, 0x411a, 0x628d, 0x010f, 0x0a5c, 0x7005, 0x7261, 0x6573, 0x6147, 0x41c3, 0x4104, 0x6523, 0xc122, 0x6300, 0x4104, 0x411a,
	0x6281, 0x4573, 0x8166, 0x4177, 0x628d, 0x41ac, 0x6d03, 0x2599, 0x4546, 0x8000, 0x0261, 0x4b08, 0x2901, 0x601c, 0x4b36, 0x2801,
	0x8029, 0x4588, 0x010a, 0x0b3c, 0x2e02, 0x0028, 0x8029, 0x4588, 0x02f2, 0x4b46, 0x5c01, 0xc122, 0x6300, 0x0102, 0x6081, 0x801f,
	0x4150, 0x25b4, 0x8013, 0x034d, 0x601c, 0x0b52, 0x7704, 0x726f, 0x0064, 0x434f, 0x4588, 0x45ae, 0x4221, 0x0423, 0x0b6a, 0x7405,
	0x6b6f, 0x6e65, 0x8020, 0x05b9, 0x0b7c, 0x6304, 0x6168, 0x0072, 0x45c2, 0x4281, 0x6103, 0x01f7, 0x6081, 0xbf00, 0x4150, 0x25d2,
	0x8008, 0x034d, 0x601c, 0x0b88, 0x2c01, 0x4221, 0x6081, 0x4133, 0x45cc, 0x4228, 0x641f, 0x0ba6, 0x6302, 0x002c, 0x4221, 0x45cc,
	0x4202, 0x801e, 0x017f, 0x40f8, 0x6803, 0x05d5, 0xcbb6, 0x6c07, 0x7469, 0x7265, 0x6c61, 0x6081, 0x40f8, 0x6703, 0x25f3, 0x6a00,
	0x45e3, 0xea00, 0x05d5, 0x05e3, 0x413f, 0xc000, 0x681f, 0x0bcc, 0x6308, 0x6d6f, 0x6970, 0x656c, 0x002c, 0x45f4, 0x05d5, 0x6081,
	0x449b, 0x2605, 0x4488, 0x6300, 0x05d5, 0x4488, 0x05fd, 0x41cb, 0x42f2, 0x800d, 0x034d, 0x6081, 0x4497, 0x2612, 0x41cb, 0x42f2,
	0x800e, 0x034d, 0x601c, 0x0bee, 0x2809, 0x696c, 0x6574, 0x6172, 0x296c, 0x40fe, 0x261c, 0x05eb, 0x601c, 0x0c26, 0x6909, 0x746e,
	0x7265, 0x7270, 0x7465, 0x44de, 0x4145, 0x2631, 0x40fe, 0x262d, 0x4161, 0x262c, 0x4488, 0x01ed, 0x05ff, 0x6103, 0x460b, 0x4488,
	0x01ed, 0x6081, 0x4281, 0x4541, 0x2638, 0x6003, 0x81d0, 0x01ef, 0x0607, 0x8c3a, 0x6307, 0x6d6f, 0x6970, 0x656c, 0x628d, 0x6381,
	0x45d5, 0x4133, 0x6147, 0x601c, 0x0c72, 0x6909, 0x6d6d, 0x6465, 0x6169, 0x6574, 0xc000, 0x42b1, 0x4172, 0x6300, 0x6903, 0x017a,
	0x0c88, 0x7306, 0x756d, 0x6764, 0x0065, 0x42b1, 0x4483, 0x8080, 0x6180, 0x064c, 0x628d, 0x6281, 0x628d, 0x4281, 0x6523, 0x4125,
	0x6147, 0x6180, 0x6147, 0x601c, 0x465a, 0x601c, 0x465a, 0x0301, 0x8022, 0x45b9, 0x4281, 0x6523, 0x0228, 0xcca0, 0x2402, 0x0022,
	0x463e, 0x4664, 0x0668, 0xccda, 0x2e02, 0x0022, 0x463e, 0x4666, 0x0668, 0x0ce6, 0x6105, 0x6f62, 0x7472, 0x40f6, 0x7b1c, 0x6180,
	0x2685, 0x4301, 0x42bc, 0x467d, 0x0686, 0x6103, 0x601c, 0x465a, 0x067f, 0xccf2, 0x6106, 0x6f62, 0x7472, 0x0022, 0x463e, 0x4687,
	0x0668, 0xc126, 0xc122, 0x4133, 0x6403, 0x8000, 0x4102, 0x8000, 0xc006, 0x641f, 0x0d12, 0x5d01, 0x40f6, 0x8172, 0x641f, 0x4d34,
	0x5b01, 0x8000, 0x8172, 0x641f, 0x4145, 0x26ae, 0x4412, 0x803f, 0x42b7, 0x42bc, 0xc400, 0x7400, 0x4691, 0x06a1, 0x601c, 0x421b,
	0x26b6, 0x4666, 0x2005, 0x6b6f, 0x2020, 0x02bc, 0x601c, 0x7281, 0xc400, 0x6e03, 0x26bd, 0x8004, 0x034d, 0x601c, 0x45c2, 0x6081,
	0x41f7, 0x26c5, 0x4623, 0x46b7, 0x06be, 0x6103, 0x81ea, 0x01ef, 0x0d3e, 0x7104, 0x6975, 0x0074, 0x4691, 0x46a1, 0x4478, 0x8d7c,
	0x432e, 0x46a4, 0x06ce, 0x601c, 0x41cb, 0x4104, 0xc006, 0x6300, 0x81ea, 0x631c, 0x81ea, 0x6403, 0xc006, 0x6403, 0x4102, 0xc122,
	0x018b, 0x0d90, 0x6508, 0x6176, 0x756c, 0x7461, 0x0065, 0x46d4, 0x4241, 0x4241, 0x6147, 0x8000, 0x40f6, 0x8000, 0x46da, 0x8d7c,
	0x432e, 0x628d, 0x4246, 0x4246, 0x46da, 0x0340, 0x4691, 0x80ee, 0xc010, 0x6403, 0x80f6, 0xc012, 0x6403, 0x8d5e, 0x88b2, 0xc014,
	0x6403, 0x81ea, 0x641f, 0xabad, 0x4157, 0x2708, 0x8016, 0x034d, 0x601c, 0x6081, 0x42b1, 0x6300, 0x449e, 0x271c, 0x42c6, 0x410a,
	0xc002, 0x6300, 0x4483, 0x4301, 0x4666, 0x200b, 0x6572, 0x6564, 0x6966, 0x656e, 0x2064, 0x02bc, 0x601c, 0x4281, 0x6c00, 0x2722,
	0x800a, 0x034d, 0x6b1c, 0x45c2, 0x44de, 0x6c00, 0x2728, 0x0607, 0x601c, 0x4723, 0x0488, 0x4dc2, 0x2701, 0x4729, 0x40fe, 0x2731,
	0x05eb, 0x601c, 0xce56, 0x5b09, 0x6f63, 0x706d, 0x6c69, 0x5d65, 0x4729, 0x05fd, 0xce64, 0x5b06, 0x6863, 0x7261, 0x005d, 0x45c8,
	0x05eb, 0xce74, 0x3b01, 0x4703, 0xe01c, 0x45d5, 0x46a1, 0x4145, 0x274b, 0x419e, 0x641f, 0x601c, 0x0e82, 0x3a01, 0x4227, 0x4221,
	0x6081, 0xc002, 0x6403, 0x42b1, 0x45d5, 0x45c2, 0x471d, 0x4709, 0x4281, 0x6523, 0x4228, 0xabad, 0x069c, 0xce98, 0x6205, 0x6765,
	0x6e69, 0x0221, 0xceba, 0x7505, 0x746e, 0x6c69, 0x413f, 0xa000, 0x6803, 0x05d5, 0xcec4, 0x6105, 0x6167, 0x6e69, 0x413f, 0x05d5,
	0x4221, 0x00fc, 0x4770, 0x076e, 0xced4, 0x6902, 0x0066, 0x4770, 0x0766, 0xcee8, 0x7404, 0x6568, 0x006e, 0x4221, 0x413f, 0x6181,
	0x6300, 0x6803, 0x017a, 0xcef2, 0x6504, 0x736c, 0x0065, 0x4772, 0x6180, 0x077d, 0xcf06, 0x7705, 0x6968, 0x656c, 0x0777, 0xcf14,
	0x7206, 0x7065, 0x6165, 0x0074, 0x6180, 0x476e, 0x077d, 0xc002, 0x6300, 0x0488, 0xcf1e, 0x7207, 0x6365, 0x7275, 0x6573, 0x4797,
	0x05fd, 0xcf34, 0x7404, 0x6961, 0x006c, 0x4797, 0x076e, 0x0f42, 0x6306, 0x6572, 0x7461, 0x0065, 0x474e, 0x6103, 0x463e, 0x400b,
	0x419e, 0x6403, 0x06a1, 0x0f4e, 0x3e05, 0x6f62, 0x7964, 0x0133, 0x628d, 0x413f, 0x4221, 0x413f, 0x4797, 0x6081, 0x4133, 0x45e3,
	0x6403, 0x05d5, 0xcf66, 0x6405, 0x656f, 0x3e73, 0x463e, 0x47b8, 0x601c, 0x0f84, 0x7608, 0x7261, 0x6169, 0x6c62, 0x0065, 0x47ac,
	0x8000, 0x05d5, 0x0f92, 0x6308, 0x6e6f, 0x7473, 0x6e61, 0x0074, 0x47ac, 0x801a, 0x45f4, 0x4221, 0x412d, 0x6403, 0x05d5, 0x0fa4,
	0x3a07, 0x6f6e, 0x616e, 0x656d, 0x4770, 0xabad, 0x069c, 0xcfbe, 0x6603, 0x726f, 0xe147, 0x45d5, 0x0221, 0xcfce, 0x6e04, 0x7865,
	0x0074, 0x463e, 0x424b, 0x05d5, 0xcfda, 0x6103, 0x7466, 0x6103, 0x4772, 0x4761, 0x619c, 0x0fe8, 0x6804, 0x6469, 0x0065, 0x4723,
	0x0656, 0x8000, 0x6147, 0x6381, 0x6281, 0x4157, 0x2809, 0x4133, 0x0803, 0x600c, 0x601c, 0x0ff6, 0x6709, 0x7465, 0x6f2d, 0x6472,
	0x7265, 0xc110, 0x4801, 0x6081, 0x412d, 0x6180, 0xc110, 0x411a, 0x413f, 0x6044, 0x6b00, 0x6081, 0x4166, 0x2820, 0x8032, 0x034d,
	0x6147, 0x0825, 0x6381, 0x6180, 0x412d, 0x424b, 0x1044, 0x6300, 0x628d, 0x601c, 0x0000, 0x660e, 0x726f, 0x6874, 0x772d, 0x726f,
	0x6c64, 0x7369, 0x0074, 0x8024, 0x601c, 0x1054, 0x7309, 0x7465, 0x6f2d, 0x6472, 0x7265, 0x6081, 0x40f6, 0x6d03, 0x2843, 0x6103,
	0x8020, 0x8001, 0x083b, 0x6081, 0x8008, 0x414b, 0x2849, 0x8031, 0x034d, 0xc110, 0x6180, 0x6147, 0x0850, 0x4172, 0x6403, 0x4133,
	0x424b, 0x109a, 0x8000, 0x017a, 0x106a, 0x6605, 0x726f, 0x6874, 0x8020, 0x4833, 0x8002, 0x083b, 0x4483, 0x41f7, 0x8080, 0x6703,
	0x6c1c, 0x42c6, 0x6081, 0x286c, 0x6081, 0x485c, 0x286a, 0x6081, 0x4490, 0x42c6, 0x42ae, 0x0862, 0x6103, 0x02bc, 0x10a8, 0x7705,
	0x726f, 0x7364, 0x4811, 0x4145, 0x287e, 0x6180, 0x6081, 0x42bc, 0x440d, 0x42c0, 0x6300, 0x4861, 0x6b00, 0x0873, 0x601c, 0x1016,
	0x6f04, 0x6c6e, 0x0079, 0x40f6, 0x083b, 0x10fe, 0x640b, 0x6665, 0x6e69, 0x7469, 0x6f69, 0x736e, 0xc110, 0x6300, 0x01a7, 0x6081,
	0x289d, 0x6b00, 0x6180, 0x6147, 0x488f, 0x6181, 0x6281, 0x6903, 0x289c, 0x410f, 0x628d, 0x023c, 0x600c, 0x601c, 0x110a, 0x2d06,
	0x726f, 0x6564, 0x0072, 0x4811, 0x488f, 0x6003, 0x083b, 0x113c, 0x2b06, 0x726f, 0x6564, 0x0072, 0x6044, 0x48a3, 0x4811, 0x628d,
	0x6180, 0x410f, 0x083b, 0x114e, 0x6506, 0x6964, 0x6f74, 0x0072, 0x438f, 0x8022, 0x08ac, 0x1166, 0x7506, 0x6470, 0x7461, 0x0065,
	0x40f6, 0xc00c, 0x641f, 0x81b6, 0x631c, 0x48c3, 0x653f, 0x1176, 0x7304, 0x7661, 0x0065, 0x8000, 0x4221, 0x7603, 0x0340, 0x118e,
	0x6605, 0x756c, 0x6873, 0xc00c, 0x6300, 0x28da, 0x8000, 0x40f6, 0x7603, 0x0340, 0x601c, 0x119e, 0x6205, 0x6f6c, 0x6b63, 0x434f,
	0x6081, 0x803f, 0x4150, 0x28e6, 0x8023, 0x034d, 0x6081, 0x81b6, 0x6403, 0x800a, 0x711f, 0x8006, 0x711f, 0x8006, 0x701f, 0x6180,
	0x48df, 0x6180, 0x48eb, 0x6523, 0x8040, 0x601c, 0x48ef, 0x06e7, 0x11b6, 0x6c04, 0x616f, 0x0064, 0x8000, 0x8010, 0x6b00, 0x6147,
	0x416c, 0x4241, 0x48f6, 0x4246, 0x410f, 0x424b, 0x1200, 0x010a, 0x807c, 0x02b7, 0x8003, 0x42c8, 0x8040, 0x802d, 0x42c9, 0x02bc,
	0x6081, 0x8002, 0x0402, 0x8020, 0x031a, 0x48df, 0x611f, 0x11f0, 0x6c04, 0x7369, 0x0074, 0x6081, 0x4915, 0x42bc, 0x490a, 0x8000,
	0x6081, 0x8010, 0x6f03, 0x292d, 0x416c, 0x4910, 0x4908, 0x48ef, 0x4303, 0x4908, 0x42bc, 0x410f, 0x0920, 0x490a, 0x010a, 0x8014,
	0x6300, 0x4100, 0x6c1c, 0x8001, 0x8014, 0x064c, 0x492f, 0x2939, 0x00fc, 0x800c, 0x6300, 0x4221, 0x6903, 0x2940, 0x8002, 0x601c,
	0x800e, 0x6300, 0x8000, 0x800e, 0x6403, 0x8000, 0x4221, 0x429e, 0x6903, 0x294c, 0x8003, 0x601c, 0x4933, 0x00fc, 0x122e, 0x6304,
	0x6c6f, 0x0064, 0x4936, 0x4145, 0x2957, 0x4116, 0x7b1c, 0x8010, 0x48df, 0x8400, 0x8000, 0x431a, 0x8012, 0x4915, 0x46f6, 0x4858,
	0xc400, 0x7400, 0x81de, 0x41ef, 0x012b, 0x4395, 0x42bc, 0x4666, 0x6509, 0x4f46, 0x5452, 0x2048, 0x2056, 0x9984, 0x8000, 0x4402,
	0x42bc, 0x4221, 0x4412, 0x441d, 0x42bc, 0x06a1, 0x4965, 0x06cc, 0x4172, 0x4488, 0x4157, 0x297d, 0x00fb, 0x0483, 0x42af, 0x4139,
	0x6147, 0x6081, 0x2993, 0x42af, 0x6081, 0x6281, 0x6180, 0x6381, 0x42af, 0x6180, 0x41b3, 0x2990, 0x42ae, 0x628d, 0x6180, 0x0978,
	0x42af, 0x6300, 0x0981, 0x600c, 0x601c, 0x6147, 0x4811, 0x6081, 0x29a6, 0x6180, 0x6281, 0x497e, 0x4145, 0x29a4, 0x6147, 0x6b00,
	0x4324, 0x628d, 0x600c, 0x601c, 0x6b00, 0x0997, 0x600c, 0x601c, 0x4995, 0x4145, 0x6c00, 0x29af, 0x4664, 0x3f03, 0x3f3f, 0x0301,
	0x6147, 0x6181, 0x6703, 0x628d, 0x4172, 0x6d03, 0x29b9, 0x6003, 0x00f6, 0x00fb, 0x40f8, 0x40f8, 0x49b0, 0x29c2, 0x4666, 0x4c03,
	0x5449, 0x601c, 0xe000, 0xe000, 0x49b0, 0x29ca, 0x4666, 0x4103, 0x554c, 0x601c, 0xe000, 0xc000, 0x49b0, 0x29d2, 0x4666, 0x4303,
	0x4c41, 0x601c, 0xe000, 0xa000, 0x49b0, 0x29da, 0x4666, 0x4203, 0x5a52, 0x601c, 0x40fb, 0x4666, 0x4203, 0x4e52, 0x601c, 0x129c,
	0x6409, 0x6365, 0x6d6f, 0x6970, 0x656c, 0x6081, 0x49ba, 0xc000, 0x6d03, 0x29ec, 0x42c6, 0x09a8, 0x611f, 0x6147, 0x6081, 0x6281,
	0x6e03, 0x29fe, 0x6081, 0x4408, 0x42c0, 0x42c6, 0x6381, 0x6081, 0x4408, 0x42c6, 0x49e5, 0x42bc, 0x4133, 0x09ee, 0x600c, 0x611f,
	0x13be, 0x7303, 0x6565, 0x45c2, 0x44b8, 0x6c00, 0x2a08, 0x0607, 0x6180, 0x6d81, 0x2a0d, 0x6103, 0x4221, 0x6147, 0x42bc, 0x42c0,
	0x42c6, 0x6081, 0x4490, 0x42c6, 0x6081, 0x42bc, 0x4488, 0x628d, 0x49ed, 0x42c6, 0x803b, 0x42b7, 0x6081, 0x4497, 0x2a28, 0x4666,
	0x200e, 0x6f63, 0x706d, 0x6c69, 0x2d65, 0x6e6f, 0x796c, 0x0020, 0x6081, 0x449b, 0x2a31, 0x4666, 0x2008, 0x6e69, 0x696c, 0x656e,
	0x0020, 0x4492, 0x2a3a, 0x4666, 0x200b, 0x6d69, 0x656d, 0x6964, 0x7461, 0x2065, 0x02bc, 0x1400, 0x2e02, 0x0073, 0x42bc, 0x42d7,
	0x6147, 0x0a45, 0x6281, 0x42df, 0x4412, 0x424b, 0x1484, 0x4666, 0x2004, 0x733c, 0x0070, 0x601c, 0x413f, 0x6147, 0x0a53, 0x6381,
	0x42c6, 0x4408, 0x4133, 0x424b, 0x149e, 0x601c, 0x1476, 0x6404, 0x6d75, 0x0070, 0x8010, 0x6523, 0x8004, 0x7003, 0x6147, 0x0a6c,
	0x42bc, 0x8010, 0x416c, 0x6181, 0x4408, 0x42c0, 0x42c6, 0x4a4c, 0x423c, 0x8002, 0x42c8, 0x4303, 0x424b, 0x14c0, 0x611f, 0x48c3,
	0x08df, 0x6081, 0x8400, 0x48ed, 0x4152, 0x2a78, 0x8018, 0x034d, 0x601c, 0x4a71, 0x48eb, 0x4a6f, 0x653f, 0x0000, 0x6201, 0x0915,
	0x14fa, 0x6c01, 0x48c3, 0x091b, 0x1500, 0x6e01, 0x8001, 0x48c5, 0x4a7f, 0x0a82, 0x1508, 0x7001, 0x40f6, 0x48c5, 0x4a7f, 0x0a82,
	0x1514, 0x6401, 0x4a79, 0x8040, 0x0913, 0x1520, 0x7801, 0x4a6f, 0x8400, 0x0913, 0x152a, 0x7301, 0x48c0, 0x08d3, 0x1534, 0x7101,
	0x8022, 0x08a3, 0x153c, 0x6501, 0x4aa0, 0x48c3, 0x48fc, 0x08b8, 0x1544, 0x6902, 0x0061, 0x48eb, 0x6523, 0x4a6f, 0x6523, 0x41cb,
	0x6103, 0x4104, 0x6523, 0x6180, 0x41cb, 0x6003, 0x4104, 0x411a, 0x4309, 0x05ab, 0x1550, 0x6901, 0x8000, 0x6180, 0x0aab
];
