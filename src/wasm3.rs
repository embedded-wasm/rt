//! Provides wasm3 wrappers for abstract [`api`] types
//! to support runtime testing in the wasm3 environment

// TODO: move this to wasm-embedded-rt-wasm3

use core::ptr;
use core::ffi::{c_void};
use log::{debug};

use crate::api::Engine;

use wasm_embedded_rt_wasm3::*;
use wasm_embedded_spec::api::{Driver};

const TASK_NAME: &'static [u8] = b"wasme\0";
const START_STR: &'static [u8] = b"_start\0";

/// WASM3 runtime errors
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature="thiserror", derive(thiserror::Error))]
pub enum Wasm3Err {
    #[cfg_attr(feature="thiserror", error("Failed to create context"))]
    Ctx,
    #[cfg_attr(feature="thiserror", error("I2C init error: {0}"))]
    I2c(i32),
    #[cfg_attr(feature="thiserror", error("SPI init error: {0}"))]
    Spi(i32),
    #[cfg_attr(feature="thiserror", error("GPIO init error: {0}"))]
    Gpio(i32),
    #[cfg_attr(feature="thiserror", error("Execution error: {0}"))]
    Exec(i32),
    #[cfg_attr(feature="thiserror", error("Driver binding error: {0}"))]
    Bind(i32),
}

/// WASM3 runtime instance
pub struct Wasm3Runtime {
    _task: wasme_task_t,
    ctx: *mut wasme_ctx_t,
}

impl Wasm3Runtime {
    /// Create new WASM3 runtime instance
    pub fn new(data: &[u8]) -> Result<Self, Wasm3Err> {
        // Setup WASME task
        let task = wasme_task_t{
            name: TASK_NAME.as_ptr() as *const c_char,
            data: data.as_ptr(),
            data_len: data.len() as u32,
        };
    
        // Initialise WASME context
        let ctx = unsafe { WASME_init(&task, 10 * 1024) };
        if ctx.is_null() {
            return Err(Wasm3Err::Ctx);
        }

        Ok(Self{
            _task: task,
            ctx,
        })
    }

    /// Bind an engine with all supported drivers
    pub fn bind_all<E: Engine>(&mut self, e: &mut E) -> Result<(), Wasm3Err> {
        Bind::<gpio_drv_t>::bind(self, e)?;
        Bind::<spi_drv_t>::bind(self, e)?;
        Bind::<i2c_drv_t>::bind(self, e)?;
        Ok(())
    }

    /// Run task in WASM3 runtime
    pub fn run(&mut self) -> Result<(), Wasm3Err> {
        let entry = START_STR.as_ptr() as *const c_char;

        let res = unsafe { WASME_run(self.ctx, entry, 0, ptr::null_mut()) };
        if res < 0 {
            return Err(Wasm3Err::Exec(res));
        }

        debug!("WASME execution complete!");

        Ok(())
    }
}

impl Drop for Wasm3Runtime {
    fn drop(&mut self) {
        unsafe { WASME_deinit(&mut self.ctx) }

        self.ctx = core::ptr::null_mut();
    }
}

/// Bind trait implemented for WASM3 runtime supported drivers
// TODO: invert `Driver` method so drivers can be passed separately and bind themselves?
pub trait Bind<'a, T> {
    fn bind<D: Driver<T>>(&'a mut self, driver: &'a mut D) -> Result<(), Wasm3Err>;
}

/// Support binding GPIO drivers
impl <'a> Bind<'a, gpio_drv_t> for Wasm3Runtime {
    fn bind<D: Driver<gpio_drv_t>>(&'a mut self, driver: &'a mut D) -> Result<(), Wasm3Err> {

        let gpio_ctx: *mut c_void = driver as *mut _ as *mut c_void;
        let gpio_drv = driver.driver();

        let res = unsafe { WASME_bind_gpio(self.ctx, &gpio_drv, gpio_ctx) };

        if res < 0 {
            Err(Wasm3Err::Bind(res))
        } else {
            Ok(())
        }
    }
}

/// Support binding SPI drivers
impl <'a> Bind<'a, spi_drv_t> for Wasm3Runtime {
    fn bind<D: Driver<spi_drv_t>>(&'a mut self, driver: &'a mut D) -> Result<(), Wasm3Err> {

        let spi_ctx: *mut c_void = driver as *mut _ as *mut c_void;
        let spi_drv = driver.driver();

        let res = unsafe { WASME_bind_spi(self.ctx, &spi_drv, spi_ctx) };

        if res < 0 {
            Err(Wasm3Err::Bind(res))
        } else {
            Ok(())
        }
    }
}

/// Support binding I2C drivers
impl <'a> Bind<'a, i2c_drv_t> for Wasm3Runtime {
    fn bind<D: Driver<i2c_drv_t>>(&'a mut self, driver: &'a mut D) -> Result<(), Wasm3Err> {

        let i2c_ctx: *mut c_void = driver as *mut _ as *mut c_void;
        let i2c_drv = driver.driver();

        let res = unsafe { WASME_bind_i2c(self.ctx, &i2c_drv, i2c_ctx) };
        
        if res < 0 {
            Err(Wasm3Err::Bind(res))
        } else {
            Ok(())
        }
    }
}
