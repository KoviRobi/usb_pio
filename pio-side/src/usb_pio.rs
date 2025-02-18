use usb_device::class_prelude::*;

pub struct UsbPio<'a, B: UsbBus> {
    read_ep: EndpointOut<'a, B>,
    write_ep: EndpointOut<'a, B>,
}

impl<'a, B: UsbBus> UsbPio<'a, B> {
    pub fn new<'alloc: 'a>(alloc: &'alloc UsbBusAllocator<B>, max_packet_size: u16) -> Self {
        UsbPio {
            read_ep: alloc.bulk(max_packet_size),
            write_ep: alloc.bulk(max_packet_size),
        }
    }
}

impl<'a, B: UsbBus> UsbClass<B> for UsbPio<'a, B> {
    fn get_configuration_descriptors(
        &self,
        writer: &mut DescriptorWriter,
    ) -> usb_device::Result<()> {
        let _ = writer;
        Ok(())
    }

    fn get_bos_descriptors(&self, writer: &mut BosWriter) -> usb_device::Result<()> {
        let _ = writer;
        Ok(())
    }

    fn get_string(&self, index: StringIndex, lang_id: LangID) -> Option<&str> {
        let _ = (index, lang_id);
        None
    }

    fn reset(&mut self) {}

    fn poll(&mut self) {}

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let _ = xfer;
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let _ = xfer;
    }

    fn endpoint_setup(&mut self, addr: EndpointAddress) {
        let _ = addr;
    }

    fn endpoint_out(&mut self, addr: EndpointAddress) {
        let _ = addr;
    }

    fn endpoint_in_complete(&mut self, addr: EndpointAddress) {
        let _ = addr;
    }

    fn get_alt_setting(&mut self, interface: InterfaceNumber) -> Option<u8> {
        let _ = interface;
        None
    }

    fn set_alt_setting(&mut self, interface: InterfaceNumber, alternative: u8) -> bool {
        let _ = (interface, alternative);
        false
    }
}
