/* Hey EMACS -*- linux-c -*- */
/* $Id: cpu_prefetch.h 2010 2006-02-25 06:45:21Z kevinkofler $ */

STATIC_INLINE uint32_t get_word_prefetch(int o) {
	uint32_t v = regs.irc;
	regs.irc = get_word(m68k_getpc() + o);
	return v;
}
STATIC_INLINE uint32_t get_long_prefetch(int o) {
	uint32_t v = get_word_prefetch(o) << 16;
	v |= get_word_prefetch(o + 2);
	return v;
}
