all: main tests

CC = clang
override CFLAGS += -g -Wno-everything -pthread -lm -O0

TEST_SRCS = CuTest.c tests.c dllists.c
SRCS = main.c dllists.c
OBJS = $(SRCS:.c=.o)
DEPS = $(SRCS:.c=.d)

TEST_OBJS = $(TEST_SRCS:.c=.o)
TEST_DEPS = $(TEST_SRCS:.c=.d)

%.d: %.c
	@set -e; rm -f $@; \
	$(CC) -MM $(CFLAGS) $< > $@.$$$$; \
	sed 's,\($*\)\.o[ :]*,\1.o $@ : ,g' < $@.$$$$ > $@; \
	rm -f $@.$$$$

include $(DEPS)
include $(TEST_DEPS)

tests: $(TEST_OBJS)
	$(CC) $(CFLAGS) $(TEST_OBJS) -o tests

main: $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -o main

clean:
	rm -f $(TEST_OBJS) $(TEST_DEPS) main
	rm -f $(OBJS) $(DEPS) main
