# [doc = "Register `CAN_MSR` reader"] pub struct R (crate :: R < CAN_MSR_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < CAN_MSR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < CAN_MSR_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < CAN_MSR_SPEC >) -> Self { R (reader) } } # [doc = "Register `CAN_MSR` writer"] pub struct W (crate :: W < CAN_MSR_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < CAN_MSR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < CAN_MSR_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < CAN_MSR_SPEC >) -> Self { W (writer) } } # [doc = "Field `INAK` reader - INAK"] pub type INAK_R = crate :: BitReader < bool > ; # [doc = "Field `SLAK` reader - SLAK"] pub type SLAK_R = crate :: BitReader < bool > ; # [doc = "Field `ERRI` reader - ERRI"] pub type ERRI_R = crate :: BitReader < bool > ; # [doc = "Field `ERRI` writer - ERRI"] pub type ERRI_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CAN_MSR_SPEC , bool , O > ; # [doc = "Field `WKUI` reader - WKUI"] pub type WKUI_R = crate :: BitReader < bool > ; # [doc = "Field `WKUI` writer - WKUI"] pub type WKUI_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CAN_MSR_SPEC , bool , O > ; # [doc = "Field `SLAKI` reader - SLAKI"] pub type SLAKI_R = crate :: BitReader < bool > ; # [doc = "Field `SLAKI` writer - SLAKI"] pub type SLAKI_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CAN_MSR_SPEC , bool , O > ; # [doc = "Field `TXM` reader - TXM"] pub type TXM_R = crate :: BitReader < bool > ; # [doc = "Field `RXM` reader - RXM"] pub type RXM_R = crate :: BitReader < bool > ; # [doc = "Field `SAMP` reader - SAMP"] pub type SAMP_R = crate :: BitReader < bool > ; # [doc = "Field `RX` reader - RX"] pub type RX_R = crate :: BitReader < bool > ; impl R { # [doc = "Bit 0 - INAK"] # [inline (always)] pub fn inak (& self) -> INAK_R { INAK_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - SLAK"] # [inline (always)] pub fn slak (& self) -> SLAK_R { SLAK_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - ERRI"] # [inline (always)] pub fn erri (& self) -> ERRI_R { ERRI_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - WKUI"] # [inline (always)] pub fn wkui (& self) -> WKUI_R { WKUI_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - SLAKI"] # [inline (always)] pub fn slaki (& self) -> SLAKI_R { SLAKI_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 8 - TXM"] # [inline (always)] pub fn txm (& self) -> TXM_R { TXM_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - RXM"] # [inline (always)] pub fn rxm (& self) -> RXM_R { RXM_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - SAMP"] # [inline (always)] pub fn samp (& self) -> SAMP_R { SAMP_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - RX"] # [inline (always)] pub fn rx (& self) -> RX_R { RX_R :: new (((self . bits >> 11) & 1) != 0) } } impl W { # [doc = "Bit 2 - ERRI"] # [inline (always)] # [must_use] pub fn erri (& mut self) -> ERRI_W < 2 > { ERRI_W :: new (self) } # [doc = "Bit 3 - WKUI"] # [inline (always)] # [must_use] pub fn wkui (& mut self) -> WKUI_W < 3 > { WKUI_W :: new (self) } # [doc = "Bit 4 - SLAKI"] # [inline (always)] # [must_use] pub fn slaki (& mut self) -> SLAKI_W < 4 > { SLAKI_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "CAN_MSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_msr](index.html) module"] pub struct CAN_MSR_SPEC ; impl crate :: RegisterSpec for CAN_MSR_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [can_msr::R](R) reader structure"] impl crate :: Readable for CAN_MSR_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [can_msr::W](W) writer structure"] impl crate :: Writable for CAN_MSR_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CAN_MSR to value 0"] impl crate :: Resettable for CAN_MSR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }