# [doc = "Register `DR37` reader"] pub struct R (crate :: R < DR37_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < DR37_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < DR37_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < DR37_SPEC >) -> Self { R (reader) } } # [doc = "Register `DR37` writer"] pub struct W (crate :: W < DR37_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < DR37_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < DR37_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < DR37_SPEC >) -> Self { W (writer) } } # [doc = "Field `D37` reader - Backup data"] pub type D37_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `D37` writer - Backup data"] pub type D37_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , DR37_SPEC , u16 , u16 , 16 , O > ; impl R { # [doc = "Bits 0:15 - Backup data"] # [inline (always)] pub fn d37 (& self) -> D37_R { D37_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - Backup data"] # [inline (always)] # [must_use] pub fn d37 (& mut self) -> D37_W < 0 > { D37_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr37](index.html) module"] pub struct DR37_SPEC ; impl crate :: RegisterSpec for DR37_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [dr37::R](R) reader structure"] impl crate :: Readable for DR37_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [dr37::W](W) writer structure"] impl crate :: Writable for DR37_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DR37 to value 0"] impl crate :: Resettable for DR37_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }