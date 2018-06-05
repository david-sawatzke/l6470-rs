pub enum WritePolicy {
    RO,
    WS,
    WH,
    WR
}

pub struct Register {
    pub address: u8,
    pub len_bit: u8,
    pub write_policy: WritePolicy,
}

pub const ABS_POS: Register = Register {
    address: 0x01,
    len_bit: 22,
    write_policy: WritePolicy::WS,
};

pub const EL_POS: Register = Register {
    address: 0x02,
    len_bit: 9,
    write_policy: WritePolicy::WS,
};

pub const MARK: Register = Register {
    address: 0x03,
    len_bit: 22,
    write_policy: WritePolicy::WR,
};

pub const SPEED: Register = Register {
    address: 0x04,
    len_bit: 20,
    write_policy: WritePolicy::RO,
};

pub const ACC: Register = Register {
    address: 0x05,
    len_bit: 12,
    write_policy: WritePolicy::WS,
};

pub const DEC: Register = Register {
    address: 0x06,
    len_bit: 12,
    write_policy: WritePolicy::WS,
};

pub const MAX_SPEED: Register = Register {
    address: 0x07,
    len_bit: 10,
    write_policy: WritePolicy::WR,
};

pub const MIN_SPEED: Register = Register {
    address: 0x08,
    len_bit: 13,
    write_policy: WritePolicy::WS,
};

pub const FS_SPD: Register = Register {
    address: 0x15,
    len_bit: 10,
    write_policy: WritePolicy::WR,
};

pub const KVAL_HOLD: Register = Register {
    address: 0x09,
    len_bit: 8,
    write_policy: WritePolicy::WR,
};

pub const KVAL_RUN: Register = Register {
    address: 0x0A,
    len_bit: 8,
    write_policy: WritePolicy::WR,
};

pub const KVAL_ACC: Register = Register {
    address: 0x0B,
    len_bit: 8,
    write_policy: WritePolicy::WR,
};

pub const KVAL_DEC: Register = Register {
    address: 0x0C,
    len_bit: 8,
    write_policy: WritePolicy::WR,
};

pub const INT_SPEED: Register = Register {
    address: 0x0D,
    len_bit: 14,
    write_policy: WritePolicy::WH,
};

pub const ST_SLP: Register = Register {
    address: 0x0E,
    len_bit: 8,
    write_policy: WritePolicy::WH,
};

pub const FN_SLP_ACC: Register = Register {
    address: 0x0F,
    len_bit: 8,
    write_policy: WritePolicy::WH,
};

pub const FN_SLP_DEC: Register = Register {
    address: 0x10,
    len_bit: 8,
    write_policy: WritePolicy::WH,
};

pub const K_TERM: Register = Register {
    address: 0x11,
    len_bit: 4,
    write_policy: WritePolicy::WR,
};


pub const ADC_OUT: Register = Register {
    address: 0x12,
    len_bit: 5,
    write_policy: WritePolicy::RO,
};


pub const OCD_TH: Register = Register {
    address: 0x13,
    len_bit: 4,
    write_policy: WritePolicy::WR,
};


pub const STALL_TH: Register = Register {
    address: 0x14,
    len_bit: 7,
    write_policy: WritePolicy::WR,
};

pub const STEP_MODE: Register = Register {
    address: 0x16,
    len_bit: 8,
    write_policy: WritePolicy::WH,
};

pub const ALARM_EN: Register = Register {
    address: 0x17,
    len_bit: 8,
    write_policy: WritePolicy::WS,
};

pub const CONFIG: Register = Register {
    address: 0x18,
    len_bit: 16,
    write_policy: WritePolicy::WH,
};

pub const STATUS: Register = Register {
    address: 0x19,
    len_bit: 16,
    write_policy: WritePolicy::RO,
};