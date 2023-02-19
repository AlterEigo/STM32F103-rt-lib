# [doc = "Register `CR2` reader"] pub struct R (crate :: R < CR2_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < CR2_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < CR2_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < CR2_SPEC >) -> Self { R (reader) } } # [doc = "Register `CR2` writer"] pub struct W (crate :: W < CR2_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < CR2_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < CR2_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < CR2_SPEC >) -> Self { W (writer) } } # [doc = "Field `ADD` reader - ADD"] pub type ADD_R = crate :: FieldReader < u8 , u8 > ; # [doc = "Field `ADD` writer - ADD"] pub type ADD_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , CR2_SPEC , u8 , u8 , 4 , O > ; # [doc = "Field `LBDL` reader - LBDL"] pub type LBDL_R = crate :: BitReader < bool > ; # [doc = "Field `LBDL` writer - LBDL"] pub type LBDL_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CR2_SPEC , bool , O > ; # [doc = "Field `LBDIE` reader - LBDIE"] pub type LBDIE_R = crate :: BitReader < bool > ; # [doc = "Field `LBDIE` writer - LBDIE"] pub type LBDIE_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CR2_SPEC , bool , O > ; # [doc = "Field `STOP` reader - STOP"] pub type STOP_R = crate :: FieldReader < u8 , u8 > ; # [doc = "Field `STOP` writer - STOP"] pub type STOP_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , CR2_SPEC , u8 , u8 , 2 , O > ; # [doc = "Field `LINEN` reader - LINEN"] pub type LINEN_R = crate :: BitReader < bool > ; # [doc = "Field `LINEN` writer - LINEN"] pub type LINEN_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CR2_SPEC , bool , O > ; impl R { # [doc = "Bits 0:3 - ADD"] # [inline (always)] pub fn add (& self) -> ADD_R { ADD_R :: new ((self . bits & 0x0f) as u8) } # [doc = "Bit 5 - LBDL"] # [inline (always)] pub fn lbdl (& self) -> LBDL_R { LBDL_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - LBDIE"] # [inline (always)] pub fn lbdie (& self) -> LBDIE_R { LBDIE_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bits 12:13 - STOP"] # [inline (always)] pub fn stop (& self) -> STOP_R { STOP_R :: new (((self . bits >> 12) & 3) as u8) } # [doc = "Bit 14 - LINEN"] # [inline (always)] pub fn linen (& self) -> LINEN_R { LINEN_R :: new (((self . bits >> 14) & 1) != 0) } } impl W { # [doc = "Bits 0:3 - ADD"] # [inline (always)] # [must_use] pub fn add (& mut self) -> ADD_W < 0 > { ADD_W :: new (self) } # [doc = "Bit 5 - LBDL"] # [inline (always)] # [must_use] pub fn lbdl (& mut self) -> LBDL_W < 5 > { LBDL_W :: new (self) } # [doc = "Bit 6 - LBDIE"] # [inline (always)] # [must_use] pub fn lbdie (& mut self) -> LBDIE_W < 6 > { LBDIE_W :: new (self) } # [doc = "Bits 12:13 - STOP"] # [inline (always)] # [must_use] pub fn stop (& mut self) -> STOP_W < 12 > { STOP_W :: new (self) } # [doc = "Bit 14 - LINEN"] # [inline (always)] # [must_use] pub fn linen (& mut self) -> LINEN_W < 14 > { LINEN_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "UART4_CR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"] pub struct CR2_SPEC ; impl crate :: RegisterSpec for CR2_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [cr2::R](R) reader structure"] impl crate :: Readable for CR2_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"] impl crate :: Writable for CR2_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CR2 to value 0"] impl crate :: Resettable for CR2_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }