//! Modbus Slave Ids.
//!
//! Slave ids are mainly used for Modbus RTU and Modbus ASCII communication but in many cases also have significance
//! for modbus TCP communication. The [SlaveId] struct is the main way of using slave ids.

/// Gets the slave id of the given modbus data.
///
/// None is returned if data contains less than 1 byte
pub fn get_slaveid(data: &[u8]) -> (Option<SlaveId>, Option<&[u8]>) {
    (
        data.get(0).map(|byte| SlaveId::new(*byte)),
        data.get(1..),
    )
}

/// Gets the slave id of the given modbus data.
///
/// # Safety
/// Providing data with less than one byte is undefined behavior
pub unsafe fn get_slaveid_unchecked(data: &[u8]) -> (SlaveId, &[u8]) {
    (
        SlaveId::new(*data.get_unchecked(0)),
        data.get_unchecked(1..),
    )
}

/// A u8 wrapper type to represent slave ids
#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlaveId(u8);

impl SlaveId {
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// The default TCP slave id is 255 decimal or 0xFF hex.
    ///
    /// Addressing in TCP is normally done through the ip address so the slave id just set to 0xFF.
    /// In many cases this rule is ignored and a certain slave address is expected.
    /// See <https://modbus.org/docs/Modbus_Messaging_Implementation_Guide_V1_0b.pdf> espacially page 23 for more details.
    pub const fn new_default_tcp() -> Self {
        Self(0xFF)
    }

    /// Creates a new broadcast slave id. The broadcast id is 0 and every slave device MUST react to it.
    pub const fn new_broadcast() -> Self {
        Self(0)
    }

    /// Check if this id is for a normal device. Only false if this is a reserved function code (> 248) or broadcast (0)
    pub const fn is_device(self) -> bool {
        !self.is_broadcast() && !self.is_reserved()
    }

    /// Checks if this function is any reserved slave address other than broadcast. (id > 248)
    pub const fn is_reserved(self) -> bool {
        self.0 >= 248
    }

    /// Checks if this is a broadcast slave id. The broadcast id is 0 and every slave device MUST react to it.
    pub const fn is_broadcast(self) -> bool {
        self.0 == 0
    }

    /// Checks if this is the default TCP slave id (0xFF)
    pub const fn is_default_tcp(self) -> bool {
        self.0 == 0xFF
    }

    /// Checks if a device with this slave id has to react to the given slave id `other`
    pub const fn must_react(self, other: SlaveId) -> bool {
        if other.0 == 0 {
            true
        } else {
            self.0 == other.0
        }
    }

    /// Gets the slave id of the given modbus data.
    ///
    /// None is returned if data contains less than 1 byte
    pub fn from_data(data: &[u8]) -> (Option<Self>, Option<&[u8]>) {
        get_slaveid(data)
    }

   /// Gets the slave id of the given modbus data.
    ///
    /// # Safety
    /// Providing data with less than one byte is undefined behavior
    pub unsafe fn from_data_unchecked(data: &[u8]) -> (Self, &[u8]) {
        get_slaveid_unchecked(data)
    }
}

impl From<u8> for SlaveId {
    fn from(sid: u8) -> Self {
        Self(sid)
    }
}

impl Into<u8> for SlaveId {
    fn into(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use crate::SlaveId;

    #[test]
    fn new10() {
        let id = SlaveId::new(10);
        assert_eq!(id.0, 10);
        assert!(id.is_device());
        assert!(!id.is_reserved());
        assert!(!id.is_default_tcp());
        assert!(!id.is_broadcast());
        assert_eq!(id, SlaveId::from(10));
    }

    #[test]
    fn device() {
        assert!(SlaveId::new(20).is_device())
    }

    #[test]
    fn broadcast() {
        assert!(SlaveId::new(0).is_broadcast());
        assert_eq!(SlaveId::new(0), SlaveId::new_broadcast());
        assert!(SlaveId::new_broadcast().is_broadcast());
    }

    #[test]
    fn tcp() {
        let tcp = SlaveId::new(0xFF);
        assert!(tcp.is_default_tcp() && tcp.is_reserved());

        assert_eq!(tcp, SlaveId::new_default_tcp());

        let tcp = SlaveId::new_default_tcp();
        assert!(tcp.is_default_tcp() && tcp.is_reserved());
    }

    #[test]
    fn all_broadcast() {
        for i in u8::MIN..=u8::MAX {
            assert!(SlaveId::new(i).must_react(SlaveId::new(0)));
            assert!(SlaveId::new(i).must_react(SlaveId::new(i)));
        }
    }

    #[test]
    fn all_device() {
        for i in 1..=247 {
            assert!(SlaveId::new(i).is_device());
            assert!(!SlaveId::new(i).is_reserved());
            assert!(SlaveId::new(i).must_react(SlaveId::new(i)));
        }
    }

    #[test]
    fn all_reserved() {
        for i in 248..=u8::MAX {
            assert!(SlaveId::new(i).is_reserved());
            assert!(!SlaveId::new(i).is_device());
            assert!(SlaveId::new(i).must_react(SlaveId::new(i)));
        }
    }
}
