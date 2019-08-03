#include "ti68k_def.h"
#include <stdarg.h>
uint16_t GUINT16_FROM_BE(uint16_t num) {
	return ((num & 0xff) >> 8) | (num << 8);
}

uint32_t GUINT32_FROM_BE(uint32_t num) {
	return ((num & 0xff000000) >> 24) | ((num & 0x00ff0000) >> 8) | ((num & 0x0000ff00) << 8) | (num << 24);
}
char *g_stpcpy(char *dest, const char *src) {
	char *d = dest;
	const char *s = src;
	do
		*d++ = *s;
	while(*s++ != '\0');

	return d - 1;
}
char *g_strconcat(const char *string1, ...) {
	uintptr_t l;
	va_list args;
	char *s;
	char *concat;
	char *ptr;

	if(!string1) return NULL;

	l = 1 + strlen(string1);
	va_start(args, string1);
	s = va_arg(args, char *);
	while(s) {
		l += strlen(s);
		s = va_arg(args, char *);
	}
	va_end(args);

	concat = malloc(l);
	ptr = concat;

	ptr = g_stpcpy(ptr, string1);
	va_start(args, string1);
	s = va_arg(args, char *);
	while(s) {
		ptr = g_stpcpy(ptr, s);
		s = va_arg(args, char *);
	}
	va_end(args);

	return concat;
}
int _g_gnulib_vasprintf(char **result, char const *format, va_list args) {
	size_t length;

	*result = vasnprintf(NULL, &length, format, args);
	if(*result == NULL) return -1;

	return length;
}
int g_vasprintf(char **string, char const *format, va_list args) {
	int len;

	len = _g_gnulib_vasprintf(string, format, args);
	if(len < 0) *string = NULL;


	return len;
}
char *g_strdup_vprintf(const char *format, va_list args) {
	char *string = NULL;

	g_vasprintf(&string, format, args);

	return string;
}
char *g_strdup_printf(const char *format, ...) {
	char *buffer;
	va_list args;

	va_start(args, format);
	buffer = g_strdup_vprintf(format, args);
	va_end(args);

	return buffer;
}
char *g_strdup(const char *str) {
	char *new_str;
	uintptr_t length;

	if(str) {
		length = strlen(str) + 1;
		new_str = malloc(length);
		memcpy(new_str, str, length);
	} else
		new_str = NULL;

	return new_str;
}
const char *g_basename(const char *file_name) {
	char *base;

	base = strrchr(file_name, '/');

	if(base) return base + 1;

	return (char *)file_name;
}