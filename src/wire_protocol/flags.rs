use coll::options::{CursorType, FindOptions};

/// Represents the bit vector of options for an OP_REPLY message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpReplyFlags {
    pub cursor_not_found: bool, // Bit 0
    pub query_failure: bool,    // Bit 1
    pub await_capable: bool,    // Bit 3

    // All bits remaining must be 0
}

impl OpReplyFlags {
    /// Constructs a new struct from a bit vector of options.
    ///
    /// # Return value
    ///
    /// Returns the newly-created struct.
    pub fn from_i32(i: i32) -> OpReplyFlags {
        let cursor_not_found = (i & 1) != 0;
        let query_failure = (i & (1 << 1)) != 0;
        let await_capable = (i & (1 << 3)) != 0;

        OpReplyFlags { cursor_not_found: cursor_not_found,
                       query_failure: query_failure,
                       await_capable: await_capable }
    }
}

/// Represents the bit vector of options for an OP_UPDATE message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpUpdateFlags {
    pub upsert: bool,        // Bit 0
    pub multi_update: bool,  // Bit 1

    // All bits remaining must be 0
}

/// Represents the bit vector of flags for an OP_INSERT message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpInsertFlags {
    pub continue_on_error: bool,  // Bit 0

    // All bits remaining must be 0
}

/// Represents the bit vector of flags for an OP_QUERY message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpQueryFlags {
    pub tailable_cursor: bool,    // Bit 1
    pub slave_ok: bool,           // Bit 2
    pub oplog_relay: bool,        // Bit 3
    pub no_cursor_timeout: bool,  // Bit 4
    pub await_data: bool,         // Bit 5
    pub exhaust: bool,            // Bit 6
    pub partial: bool,            // Bit 7
    // All bits remaining must be 0
}

impl OpUpdateFlags {
    /// Constructs a new struct with all flags set to false.
    ///
    /// # Return value
    ///
    /// Returns the newly-created struct.
    pub fn no_flags() -> OpUpdateFlags {
        OpUpdateFlags { upsert: false, multi_update: false }
    }

    /// Gets the actual bit vector that the struct represents.
    ///
    /// # Return value
    ///
    /// Returns the bit vector as an i32.
    pub fn to_i32(&self) -> i32 {
        let mut i = 0 as i32;

        if self.upsert {
            i = 1;
        }

        if self.multi_update {
            i |= 1 << 1;
        }

        i
    }
}

impl OpInsertFlags {
    /// Constructs a new struct with all flags set to false.
    ///
    /// # Return value
    ///
    /// Returns the newly-created struct.
    pub fn no_flags() -> OpInsertFlags {
        OpInsertFlags { continue_on_error: false }
    }

    /// Gets the actual bit vector that the struct represents.
    ///
    /// # Return value
    ///
    /// Returns the bit vector as an i32.
    pub fn to_i32(&self) -> i32 {
        if self.continue_on_error {
            1
        } else {
            0
        }
    }
}

impl OpQueryFlags {
    /// Constructs a new struct with all flags set to false.
    ///
    /// # Return value
    ///
    /// Returns the newly-created struct.
    pub fn no_flags() -> OpQueryFlags {
        OpQueryFlags { tailable_cursor: false, slave_ok: false,
                       oplog_relay: false, no_cursor_timeout: false,
                       await_data: false, exhaust: false, partial: false }
    }

    /// Constructs a new struct with flags based on a FindOptions struct.
    ///
    /// # Arguments
    ///
    /// options - Struct whose fields contain the flags to initialize the new
    ///           OpQueryFlags with
    ///
    /// # Return value
    ///
    /// Returns the newly created OpQueryFlags struct.
    pub fn with_find_options<'a>(options: &'a FindOptions) -> OpQueryFlags {
        OpQueryFlags {
            tailable_cursor: options.cursor_type != CursorType::NonTailable,
            slave_ok: false,
            oplog_relay: options.op_log_replay,
            no_cursor_timeout: options.no_cursor_timeout,
            await_data: options.cursor_type == CursorType::TailableAwait,
            exhaust: false,
            partial: options.allow_partial_results,
        }
    }

    /// Gets the actual bit vector that the struct represents.
    ///
    /// # Return value
    ///
    /// Returns the bit vector as an i32.
    pub fn to_i32(&self) -> i32 {
        let mut i = 0 as i32;

        if self.tailable_cursor {
            i |= 1 << 1;
        }

        if self.slave_ok {
            i |= 1 << 2;
        }

        if self.oplog_relay {
            i |= 1 << 3;
        }

        if self.no_cursor_timeout {
            i |= 1 << 4;
        }

        if self.await_data {
            i |= 1 << 5;
        }

        if self.exhaust {
            i |= 1 << 6;
        }

        if self.partial {
            i |= 1 << 7;
        }

        i
    }
}
