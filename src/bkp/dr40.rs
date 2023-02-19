# [doc = "Register `DR40` reader"] pub struct R (crate :: R < DR40_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < DR40_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < DR40_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < DR40_SPEC >) -> Self { R (reader) } } # [doc = "Register `DR40` writer"] pub struct W (crate :: W < DR40_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < DR40_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < DR40_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < DR40_SPEC >) -> Self { W (writer) } } # [doc = "Field `D40` reader - Backup data"] pub type D40_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `D40` writer - Backup data"] pub type D40_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , DR40_SPEC , u16 , u16 , 16 , O > ; impl R { # [doc = "Bits 0:15 - Backup data"] # [inline (always)] pub fn d40 (& self) -> D40_R { D40_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - Backup data"] # [inline (always)] # [must_use] pub fn d40 (& mut self) -> D40_W < 0 > { D40_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr40](index.html) module"] pub struct DR40_SPEC ; impl crate :: RegisterSpec for DR40_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [dr40::R](R) reader structure"] impl crate :: Readable for DR40_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [dr40::W](W) writer structure"] impl crate :: Writable for DR40_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DR40 to value 0"] impl crate :: Resettable for DR40_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }