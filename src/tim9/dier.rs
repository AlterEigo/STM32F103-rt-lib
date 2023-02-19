# [doc = "Register `DIER` reader"] pub struct R (crate :: R < DIER_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < DIER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < DIER_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < DIER_SPEC >) -> Self { R (reader) } } # [doc = "Register `DIER` writer"] pub struct W (crate :: W < DIER_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < DIER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < DIER_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < DIER_SPEC >) -> Self { W (writer) } } # [doc = "Field `UIE` reader - Update interrupt enable"] pub type UIE_R = crate :: BitReader < bool > ; # [doc = "Field `UIE` writer - Update interrupt enable"] pub type UIE_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , DIER_SPEC , bool , O > ; # [doc = "Field `CC1IE` reader - Capture/Compare 1 interrupt enable"] pub type CC1IE_R = crate :: BitReader < bool > ; # [doc = "Field `CC1IE` writer - Capture/Compare 1 interrupt enable"] pub type CC1IE_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , DIER_SPEC , bool , O > ; # [doc = "Field `CC2IE` reader - Capture/Compare 2 interrupt enable"] pub type CC2IE_R = crate :: BitReader < bool > ; # [doc = "Field `CC2IE` writer - Capture/Compare 2 interrupt enable"] pub type CC2IE_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , DIER_SPEC , bool , O > ; # [doc = "Field `TIE` reader - Trigger interrupt enable"] pub type TIE_R = crate :: BitReader < bool > ; # [doc = "Field `TIE` writer - Trigger interrupt enable"] pub type TIE_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , DIER_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - Update interrupt enable"] # [inline (always)] pub fn uie (& self) -> UIE_R { UIE_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Capture/Compare 1 interrupt enable"] # [inline (always)] pub fn cc1ie (& self) -> CC1IE_R { CC1IE_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Capture/Compare 2 interrupt enable"] # [inline (always)] pub fn cc2ie (& self) -> CC2IE_R { CC2IE_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 6 - Trigger interrupt enable"] # [inline (always)] pub fn tie (& self) -> TIE_R { TIE_R :: new (((self . bits >> 6) & 1) != 0) } } impl W { # [doc = "Bit 0 - Update interrupt enable"] # [inline (always)] # [must_use] pub fn uie (& mut self) -> UIE_W < 0 > { UIE_W :: new (self) } # [doc = "Bit 1 - Capture/Compare 1 interrupt enable"] # [inline (always)] # [must_use] pub fn cc1ie (& mut self) -> CC1IE_W < 1 > { CC1IE_W :: new (self) } # [doc = "Bit 2 - Capture/Compare 2 interrupt enable"] # [inline (always)] # [must_use] pub fn cc2ie (& mut self) -> CC2IE_W < 2 > { CC2IE_W :: new (self) } # [doc = "Bit 6 - Trigger interrupt enable"] # [inline (always)] # [must_use] pub fn tie (& mut self) -> TIE_W < 6 > { TIE_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dier](index.html) module"] pub struct DIER_SPEC ; impl crate :: RegisterSpec for DIER_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [dier::R](R) reader structure"] impl crate :: Readable for DIER_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [dier::W](W) writer structure"] impl crate :: Writable for DIER_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DIER to value 0"] impl crate :: Resettable for DIER_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }