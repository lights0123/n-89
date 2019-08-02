/*
 * UAE - The Un*x Amiga Emulator
 *
 * Try to include the right system headers and get other system-specific
 * stuff right & other collected kludges.
 *
 * If you think about modifying this, think twice. Some systems rely on
 * the exact order of the #include statements. That's also the reason
 * why everything gets included unconditionally regardless of whether
 * it's actually needed by the .c file.
 *
 * Copyright 1996, 1997 Bernd Schmidt
 * $Id: sysdeps.h 2556 2007-06-24 05:05:05Z kevinkofler $
 */

#ifdef _MSC_VER
#pragma warning(disable : 4142)
#endif

#define REGPARAM
#define REGPARAM2


// This portion copied from the ld-tigcc generic.h
// Copyright (C) 2002-2004 Sebastian Reichelt
// licensed under the GPL

// Attempt to auto-detect I1, I2, I4, SI1, SI2 and SI4 based on <limits.h>.
// It is a good idea to double-check these definitions on every new system
// you compile on.

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <limits.h>

// Make sure that the character types take exactly 1 byte.
#if UCHAR_MAX != 0xFF
#error Need 1-byte unsigned char type.
#endif /* UCHAR_MAX != 0xFF */
#if SCHAR_MIN != (-0x80) || SCHAR_MAX != 0x7F
#error Need 1-byte signed char type.
#endif /* UCHAR_MAX != 0xFF */

#undef uae_s64
#undef uae_u64

// This part coded by Kevin Kofler. Copyright (C) Kevin Kofler, 2005.
// This is hard to do right because too long numbers might cause a parse error.
#if defined(ULONG_LONG_MAX) || defined(ULLONG_MAX)
#if ULONG_LONG_MAX == 0xFFFFFFFFFFFFFFFFll || ULLONG_MAX == 0xFFFFFFFFFFFFFFFFll
#define uae_s64 long long
#define uae_u64 unsigned long long
#define VAL64(a) (a##LL)
#define UVAL64(a) (a##uLL)
#endif
#endif

#ifndef uae_u64
#if defined(_MSC_VER)
#define uae_s64 __int64
#define uae_u64 unsigned __int64
#define VAL64(a) (a)
#define UVAL64(a) (a)
#elif ULONG_MAX > 0xFFFFFFFF
#if ULONG_MAX == 0xFFFFFFFFFFFFFFFF
#define uae_s64 long
#define uae_u64 unsigned long
#define VAL64(a) (a##l)
#define UVAL64(a) (a##ul)
#endif
#endif
#endif

#ifdef HAVE_STRDUP
#define my_strdup strdup
#else
extern char *my_strdup(const char *s);
#endif

#include <stddef.h>
extern void *xmalloc(size_t);
extern void *xcalloc(size_t, size_t);

/* We can only rely on GNU C getting enums right. Mickeysoft VSC++ is known
 * to have problems, and it's likely that other compilers choke too. */
#ifdef __GNUC__
#define ENUMDECL typedef enum
#define ENUMNAME(name) name

#else
#define ENUMDECL enum
#define ENUMNAME(name) \
	;                  \
	typedef int name
#endif

#ifdef X86_ASSEMBLY
#define ASM_SYM_FOR_FUNC(a) __asm__(a)
#else
#define ASM_SYM_FOR_FUNC(a)
#endif

#if defined USE_COMPILER
#undef NO_PREFETCH_BUFFER
#undef NO_EXCEPTION_3
#define NO_EXCEPTION_3
#define NO_PREFETCH_BUFFER
#endif

#ifndef STATIC_INLINE
#if __GNUC__ - 1 > 1 && __GNUC_MINOR__ - 1 >= 0
#define STATIC_INLINE static __inline__ __attribute__((always_inline))
#define NOINLINE __attribute__((noinline))
#else
#define STATIC_INLINE static __inline__
#define NOINLINE
#endif
#endif

#ifdef _MSC_VER
#define __inline__ __inline
#endif

/* Every Amiga hardware clock cycle takes this many "virtual" cycles.  This
   used to be hardcoded as 1, but using higher values allows us to time some
   stuff more precisely.
   512 is the official value from now on - it can't change, unless we want
   _another_ config option "finegrain2_m68k_speed".

   We define this value here rather than in events.h so that gencpu.c sees
   it.  */
#define CYCLE_UNIT 1 /* (TiEmu patch, was 512) */

/* This one is used by cfgfile.c.  We could reduce the CYCLE_UNIT back to 1,
   I'm not 100% sure this code is bug free yet.  */
#define OFFICIAL_CYCLE_UNIT 1 /* (TiEmu patch, was 512) */

/*
 * You can specify numbers from 0 to 5 here. It is possible that higher
 * numbers will make the CPU emulation slightly faster, but if the setting
 * is too high, you will run out of memory while compiling.
 * Best to leave this as it is.
 */
#define CPU_EMU_SIZE 0
#ifndef STATIC_INLINE
#if __GNUC__ - 1 > 1 && __GNUC_MINOR__ - 1 >= 0
#define STATIC_INLINE static __inline__ __attribute__((always_inline))
#define NOINLINE __attribute__((noinline))
#else
#define STATIC_INLINE static __inline__
#define NOINLINE
#endif
#endif

/* Unoptimised version: always call the _n() function. */

#define _G_NEW(struct_type, n_structs, func) \
	((struct_type *)g_##func##_n((n_structs), sizeof(struct_type)))
#define _G_RENEW(struct_type, mem, n_structs, func) \
	((struct_type *)g_##func##_n(mem, (n_structs), sizeof(struct_type)))

/**
 * g_new:
 * @struct_type: the type of the elements to allocate
 * @n_structs: the number of elements to allocate
 *
 * Allocates @n_structs elements of type @struct_type.
 * The returned pointer is cast to a pointer to the given type.
 * If @n_structs is 0 it returns %NULL.
 * Care is taken to avoid overflow when calculating the size of the allocated block.
 *
 * Since the returned pointer is already casted to the right type,
 * it is normally unnecessary to cast it explicitly, and doing
 * so might hide memory allocation errors.
 *
 * Returns: a pointer to the allocated memory, cast to a pointer to @struct_type
 */
#define g_new(struct_type, n_structs) _G_NEW(struct_type, n_structs, malloc)
/**
 * g_new0:
 * @struct_type: the type of the elements to allocate.
 * @n_structs: the number of elements to allocate.
 *
 * Allocates @n_structs elements of type @struct_type, initialized to 0's.
 * The returned pointer is cast to a pointer to the given type.
 * If @n_structs is 0 it returns %NULL.
 * Care is taken to avoid overflow when calculating the size of the allocated block.
 *
 * Since the returned pointer is already casted to the right type,
 * it is normally unnecessary to cast it explicitly, and doing
 * so might hide memory allocation errors.
 *
 * Returns: a pointer to the allocated memory, cast to a pointer to @struct_type.
 */
#define g_new0(struct_type, n_structs) _G_NEW(struct_type, n_structs, malloc0)