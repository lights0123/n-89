#include "logging.h"
#include <stdarg.h>
#include <stdio.h> /* printf */
void tiemu_debug(const char *format, ...) {
	va_list args;
	va_start(args, format);
	printf("[DBG] : ");
	vprintf(format, args);

	va_end(args);
}
void tiemu_info(const char *format, ...) {
	va_list args;
	va_start(args, format);
	printf("[INFO]: ");
	vprintf(format, args);

	va_end(args);
}
void tiemu_message(const char *format, ...) {
	va_list args;
	va_start(args, format);
	printf("[MSG] : ");
	vprintf(format, args);

	va_end(args);
}
void tiemu_warning(const char *format, ...) {
	va_list args;
	va_start(args, format);
	printf("[WARN]: ");
	vprintf(format, args);

	va_end(args);
}
void tiemu_critical(const char *format, ...) {
	va_list args;
	va_start(args, format);
	printf("[CRIT]: ");
	vprintf(format, args);

	va_end(args);
}
void tiemu_error(const char *format, ...) {
	va_list args;
	va_start(args, format);
	printf("[ERR] : ");
	vprintf(format, args);

	va_end(args);
}