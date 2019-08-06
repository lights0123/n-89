#include <stdint.h>
#include <time.h>
char *      vasnprintf (char *, size_t *, const char *, __VALIST);
/**
  This should be the FIRST function to call (unless the 'params'
  structure has been properly initialized).
 */
int ti68k_config_load_default(void);
/**
    This should be the SECOND function to call.
    Load a ROM image (images.c).
*/
int ti68k_load_image(const char *filename);
/**
  This is the THIRD function to call for completely initializing the
  emulation engine.
*/
int ti68k_init(void);
/**
  This should be the FOURTH function to call.
  It simply resets the hardware engine.
*/
int ti68k_reset(void);
/**
  Close the library by exiting the emulation engine
  (free resources).
*/
int ti68k_exit(void);
/**
  Do 'n' instructions (up to 'maxcycles', unless set to 0).

  Returned value:
  - 1 if breakpoint has been encountered,
  - 2 if trace,
  - 0 otherwise.
*/
int hw_m68k_run(int n, unsigned maxcycles);
void ti68k_kbd_set_key(int key, int active);

typedef struct {
	time_t s;
	int ms;
} TTIME;
/** If this structure is modified, the SAV_REVISION number (state.c)
    has to be incremented.
*/
typedef struct {
	/** misc (non hardware pseudo-constants) */
	int calc_type;

	/** RAM size */
	int ram_size;
	/** ROM size */
	int rom_size;
	/** HWx io size */
	int io_size;
	/** HW2 io size */
	int io2_size;
	/** HW3 io size */
	int io3_size;

	/** ROM base address */
	uint32_t rom_base;
	/** ROM type */
	int rom_flash;
	/** ROM/AMS version */
	char rom_version[5];

	/** HW1/2/3/4 */
	int hw_type;

	/** ROM v1.x(y) */
	int ti92v1;
	/** ROM v2.x */
	int ti92v2;

	/** LCD physical width */
	int lcd_w;
	/** LCD physical height */
	int lcd_h;

	// keyboard

	int on_key;

	// lcd

	/** LCD address (as $4c00) */
	uint32_t lcd_adr;
	/** direct pointer to LCD in PC RAM */
	char *lcd_ptr;
	int contrast;
	/** LCD logical width */
	int log_w;
	/** LCD logical height */
	int log_h;
	int on_off;
	/** used by grayscales */
	unsigned long lcd_tick;

	// memory

	/** ROM */
	uint8_t *rom;
	/** RAM */
	uint8_t *ram;
	/** HW1/2/3 i/o ports */
	uint8_t *io;
	/** HW2/3   i/o ports */
	uint8_t *io2;
	/** HW3	   i/o ports */
	uint8_t *io3;
	/** unused */
	uint8_t *unused;

	/** SSP at vector #0 */
	uint32_t initial_ssp;
	/** PC  at vector #1 */
	uint32_t initial_pc;

	// timer

	/** Current timer value */
	uint8_t timer_value;
	/** Value to reload */
	uint8_t timer_init;

	// rtc (hw2)

	/** RTC value */
	uint8_t rtc_value;

	// rtc (hw3)

	/** time reference */
	TTIME rtc3_ref;
	/** time value when */
	TTIME rtc3_beg;
	/** clock is load */
	TTIME rtc3_load;

	// protection

	/** hw protection state */
	int protect;
	/** archive memory limit */
	uint32_t archive_limit;
	/** RAM page execution protection bitmask */
	int ram_exec[64];

} Ti68kHardware;
typedef enum {
	TIKEY_DOWN,
	TIKEY_RIGHT,
	TIKEY_UP,
	TIKEY_LEFT,
	TIKEY_HAND,
	TIKEY_SHIFT,
	TIKEY_DIAMOND,
	TIKEY_2ND,
	TIKEY_3,
	TIKEY_2,
	TIKEY_1,
	TIKEY_F8,
	TIKEY_W,
	TIKEY_S,
	TIKEY_Z,
	TIKEY_6,
	TIKEY_5,
	TIKEY_4,
	TIKEY_F3,
	TIKEY_E,
	TIKEY_D,
	TIKEY_X,
	TIKEY_9,
	TIKEY_8,
	TIKEY_7,
	TIKEY_F7,
	TIKEY_R,
	TIKEY_F,
	TIKEY_C,
	TIKEY_STORE,
	TIKEY_COMMA,
	TIKEY_PARIGHT,
	TIKEY_PALEFT,
	TIKEY_F2,
	TIKEY_T,
	TIKEY_G,
	TIKEY_V,
	TIKEY_SPACE,
	TIKEY_TAN,
	TIKEY_COS,
	TIKEY_SIN,
	TIKEY_F6,
	TIKEY_Y,
	TIKEY_H,
	TIKEY_B,
	TIKEY_DIVIDE,
	TIKEY_P,
	TIKEY_ENTER2,
	TIKEY_LN,
	TIKEY_F1,
	TIKEY_U,
	TIKEY_J,
	TIKEY_N,
	TIKEY_POWER,
	TIKEY_MULTIPLY,
	TIKEY_APPS,
	TIKEY_CLEAR,
	TIKEY_F5,
	TIKEY_I,
	TIKEY_K,
	TIKEY_M,
	TIKEY_EQUALS,
	TIKEY_NU,
	TIKEY_ESCAPE,
	TIKEY_MODE,
	TIKEY_PLUS,
	TIKEY_O,
	TIKEY_L,
	TIKEY_THETA,
	TIKEY_BACKSPACE,
	TIKEY_NEGATE,
	TIKEY_PERIOD,
	TIKEY_0,
	TIKEY_F4,
	TIKEY_Q,
	TIKEY_A,
	TIKEY_ENTER1,
	TIKEY_MINUS,
	TIKEY_ON,
	TIKEY_ALPHA,
	TIKEY_EE,
	TIKEY_CATALOG,
	TIKEY_HOME,
	TIKEY_PIPE,
	TIKEY_VOID,
	MAX_TIKEYS
} TiKey;
extern Ti68kHardware tihw;
