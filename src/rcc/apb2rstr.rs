# [doc = "Register `APB2RSTR` reader"] pub struct R (crate :: R < APB2RSTR_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < APB2RSTR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < APB2RSTR_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < APB2RSTR_SPEC >) -> Self { R (reader) } } # [doc = "Register `APB2RSTR` writer"] pub struct W (crate :: W < APB2RSTR_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < APB2RSTR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < APB2RSTR_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < APB2RSTR_SPEC >) -> Self { W (writer) } } # [doc = "Field `AFIORST` reader - Alternate function I/O reset"] pub type AFIORST_R = crate :: BitReader < bool > ; # [doc = "Field `AFIORST` writer - Alternate function I/O reset"] pub type AFIORST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `IOPARST` reader - IO port A reset"] pub type IOPARST_R = crate :: BitReader < bool > ; # [doc = "Field `IOPARST` writer - IO port A reset"] pub type IOPARST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `IOPBRST` reader - IO port B reset"] pub type IOPBRST_R = crate :: BitReader < bool > ; # [doc = "Field `IOPBRST` writer - IO port B reset"] pub type IOPBRST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `IOPCRST` reader - IO port C reset"] pub type IOPCRST_R = crate :: BitReader < bool > ; # [doc = "Field `IOPCRST` writer - IO port C reset"] pub type IOPCRST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `IOPDRST` reader - IO port D reset"] pub type IOPDRST_R = crate :: BitReader < bool > ; # [doc = "Field `IOPDRST` writer - IO port D reset"] pub type IOPDRST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `IOPERST` reader - IO port E reset"] pub type IOPERST_R = crate :: BitReader < bool > ; # [doc = "Field `IOPERST` writer - IO port E reset"] pub type IOPERST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `IOPFRST` reader - IO port F reset"] pub type IOPFRST_R = crate :: BitReader < bool > ; # [doc = "Field `IOPFRST` writer - IO port F reset"] pub type IOPFRST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `IOPGRST` reader - IO port G reset"] pub type IOPGRST_R = crate :: BitReader < bool > ; # [doc = "Field `IOPGRST` writer - IO port G reset"] pub type IOPGRST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `ADC1RST` reader - ADC 1 interface reset"] pub type ADC1RST_R = crate :: BitReader < bool > ; # [doc = "Field `ADC1RST` writer - ADC 1 interface reset"] pub type ADC1RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `ADC2RST` reader - ADC 2 interface reset"] pub type ADC2RST_R = crate :: BitReader < bool > ; # [doc = "Field `ADC2RST` writer - ADC 2 interface reset"] pub type ADC2RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `TIM1RST` reader - TIM1 timer reset"] pub type TIM1RST_R = crate :: BitReader < bool > ; # [doc = "Field `TIM1RST` writer - TIM1 timer reset"] pub type TIM1RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `SPI1RST` reader - SPI 1 reset"] pub type SPI1RST_R = crate :: BitReader < bool > ; # [doc = "Field `SPI1RST` writer - SPI 1 reset"] pub type SPI1RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `TIM8RST` reader - TIM8 timer reset"] pub type TIM8RST_R = crate :: BitReader < bool > ; # [doc = "Field `TIM8RST` writer - TIM8 timer reset"] pub type TIM8RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `USART1RST` reader - USART1 reset"] pub type USART1RST_R = crate :: BitReader < bool > ; # [doc = "Field `USART1RST` writer - USART1 reset"] pub type USART1RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `ADC3RST` reader - ADC 3 interface reset"] pub type ADC3RST_R = crate :: BitReader < bool > ; # [doc = "Field `ADC3RST` writer - ADC 3 interface reset"] pub type ADC3RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `TIM9RST` reader - TIM9 timer reset"] pub type TIM9RST_R = crate :: BitReader < bool > ; # [doc = "Field `TIM9RST` writer - TIM9 timer reset"] pub type TIM9RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `TIM10RST` reader - TIM10 timer reset"] pub type TIM10RST_R = crate :: BitReader < bool > ; # [doc = "Field `TIM10RST` writer - TIM10 timer reset"] pub type TIM10RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; # [doc = "Field `TIM11RST` reader - TIM11 timer reset"] pub type TIM11RST_R = crate :: BitReader < bool > ; # [doc = "Field `TIM11RST` writer - TIM11 timer reset"] pub type TIM11RST_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , APB2RSTR_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - Alternate function I/O reset"] # [inline (always)] pub fn afiorst (& self) -> AFIORST_R { AFIORST_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 2 - IO port A reset"] # [inline (always)] pub fn ioparst (& self) -> IOPARST_R { IOPARST_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - IO port B reset"] # [inline (always)] pub fn iopbrst (& self) -> IOPBRST_R { IOPBRST_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - IO port C reset"] # [inline (always)] pub fn iopcrst (& self) -> IOPCRST_R { IOPCRST_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - IO port D reset"] # [inline (always)] pub fn iopdrst (& self) -> IOPDRST_R { IOPDRST_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - IO port E reset"] # [inline (always)] pub fn ioperst (& self) -> IOPERST_R { IOPERST_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - IO port F reset"] # [inline (always)] pub fn iopfrst (& self) -> IOPFRST_R { IOPFRST_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bit 8 - IO port G reset"] # [inline (always)] pub fn iopgrst (& self) -> IOPGRST_R { IOPGRST_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - ADC 1 interface reset"] # [inline (always)] pub fn adc1rst (& self) -> ADC1RST_R { ADC1RST_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - ADC 2 interface reset"] # [inline (always)] pub fn adc2rst (& self) -> ADC2RST_R { ADC2RST_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - TIM1 timer reset"] # [inline (always)] pub fn tim1rst (& self) -> TIM1RST_R { TIM1RST_R :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 12 - SPI 1 reset"] # [inline (always)] pub fn spi1rst (& self) -> SPI1RST_R { SPI1RST_R :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - TIM8 timer reset"] # [inline (always)] pub fn tim8rst (& self) -> TIM8RST_R { TIM8RST_R :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bit 14 - USART1 reset"] # [inline (always)] pub fn usart1rst (& self) -> USART1RST_R { USART1RST_R :: new (((self . bits >> 14) & 1) != 0) } # [doc = "Bit 15 - ADC 3 interface reset"] # [inline (always)] pub fn adc3rst (& self) -> ADC3RST_R { ADC3RST_R :: new (((self . bits >> 15) & 1) != 0) } # [doc = "Bit 19 - TIM9 timer reset"] # [inline (always)] pub fn tim9rst (& self) -> TIM9RST_R { TIM9RST_R :: new (((self . bits >> 19) & 1) != 0) } # [doc = "Bit 20 - TIM10 timer reset"] # [inline (always)] pub fn tim10rst (& self) -> TIM10RST_R { TIM10RST_R :: new (((self . bits >> 20) & 1) != 0) } # [doc = "Bit 21 - TIM11 timer reset"] # [inline (always)] pub fn tim11rst (& self) -> TIM11RST_R { TIM11RST_R :: new (((self . bits >> 21) & 1) != 0) } } impl W { # [doc = "Bit 0 - Alternate function I/O reset"] # [inline (always)] # [must_use] pub fn afiorst (& mut self) -> AFIORST_W < 0 > { AFIORST_W :: new (self) } # [doc = "Bit 2 - IO port A reset"] # [inline (always)] # [must_use] pub fn ioparst (& mut self) -> IOPARST_W < 2 > { IOPARST_W :: new (self) } # [doc = "Bit 3 - IO port B reset"] # [inline (always)] # [must_use] pub fn iopbrst (& mut self) -> IOPBRST_W < 3 > { IOPBRST_W :: new (self) } # [doc = "Bit 4 - IO port C reset"] # [inline (always)] # [must_use] pub fn iopcrst (& mut self) -> IOPCRST_W < 4 > { IOPCRST_W :: new (self) } # [doc = "Bit 5 - IO port D reset"] # [inline (always)] # [must_use] pub fn iopdrst (& mut self) -> IOPDRST_W < 5 > { IOPDRST_W :: new (self) } # [doc = "Bit 6 - IO port E reset"] # [inline (always)] # [must_use] pub fn ioperst (& mut self) -> IOPERST_W < 6 > { IOPERST_W :: new (self) } # [doc = "Bit 7 - IO port F reset"] # [inline (always)] # [must_use] pub fn iopfrst (& mut self) -> IOPFRST_W < 7 > { IOPFRST_W :: new (self) } # [doc = "Bit 8 - IO port G reset"] # [inline (always)] # [must_use] pub fn iopgrst (& mut self) -> IOPGRST_W < 8 > { IOPGRST_W :: new (self) } # [doc = "Bit 9 - ADC 1 interface reset"] # [inline (always)] # [must_use] pub fn adc1rst (& mut self) -> ADC1RST_W < 9 > { ADC1RST_W :: new (self) } # [doc = "Bit 10 - ADC 2 interface reset"] # [inline (always)] # [must_use] pub fn adc2rst (& mut self) -> ADC2RST_W < 10 > { ADC2RST_W :: new (self) } # [doc = "Bit 11 - TIM1 timer reset"] # [inline (always)] # [must_use] pub fn tim1rst (& mut self) -> TIM1RST_W < 11 > { TIM1RST_W :: new (self) } # [doc = "Bit 12 - SPI 1 reset"] # [inline (always)] # [must_use] pub fn spi1rst (& mut self) -> SPI1RST_W < 12 > { SPI1RST_W :: new (self) } # [doc = "Bit 13 - TIM8 timer reset"] # [inline (always)] # [must_use] pub fn tim8rst (& mut self) -> TIM8RST_W < 13 > { TIM8RST_W :: new (self) } # [doc = "Bit 14 - USART1 reset"] # [inline (always)] # [must_use] pub fn usart1rst (& mut self) -> USART1RST_W < 14 > { USART1RST_W :: new (self) } # [doc = "Bit 15 - ADC 3 interface reset"] # [inline (always)] # [must_use] pub fn adc3rst (& mut self) -> ADC3RST_W < 15 > { ADC3RST_W :: new (self) } # [doc = "Bit 19 - TIM9 timer reset"] # [inline (always)] # [must_use] pub fn tim9rst (& mut self) -> TIM9RST_W < 19 > { TIM9RST_W :: new (self) } # [doc = "Bit 20 - TIM10 timer reset"] # [inline (always)] # [must_use] pub fn tim10rst (& mut self) -> TIM10RST_W < 20 > { TIM10RST_W :: new (self) } # [doc = "Bit 21 - TIM11 timer reset"] # [inline (always)] # [must_use] pub fn tim11rst (& mut self) -> TIM11RST_W < 21 > { TIM11RST_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "APB2 peripheral reset register (RCC_APB2RSTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"] pub struct APB2RSTR_SPEC ; impl crate :: RegisterSpec for APB2RSTR_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [apb2rstr::R](R) reader structure"] impl crate :: Readable for APB2RSTR_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"] impl crate :: Writable for APB2RSTR_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets APB2RSTR to value 0"] impl crate :: Resettable for APB2RSTR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }