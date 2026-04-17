use std::rc::Rc;

fn make_targets(raw: &[(&str, &[&str])]) -> crate::Targets {
    raw.iter()
        .map(|(k, vs)| {
            (
                Rc::new(k.to_string()),
                vs.iter().map(|v| Rc::new(v.to_string())).collect(),
            )
        })
        .collect()
}

#[test]
fn test_parse() {
    let mut makefile = "# GNU Make 4.3
# Built for x86_64-pc-linux-gnu
# Copyright (C) 1988-2020 Free Software Foundation, Inc.
# License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
# This is free software: you are free to change and redistribute it.
# There is NO WARRANTY, to the extent permitted by law.
# Make data base, printed on Fri Apr 17 22:06:28 2026
# Variables
# environment
TMUX_PANE = %2
# default
PREPROCESS.S = $(CC) -E $(CPPFLAGS)
# environment
JAVA_HOME = /home/aj/.sdkman/candidates/java/current
# default
COMPILE.m = $(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c
# default
ARFLAGS = rv
# default
AS = as
# environment
GNOME_SHELL_SESSION_MODE = pop
# environment
SDKMAN_PLATFORM = linuxx64
# environment
SSH_AGENT_LAUNCHER = gnome-keyring
# default
AR = ar
# environment
NVIM = /run/user/1000/nvim.56088.0
# environment
MASON = /home/aj/.local/share/nvim/mason
# environment
CMD_DURATION_MS = 9
# environment
STARSHIP_SESSION_KEY = UdQMRCeLFIBMqSQl
# default
OBJC = cc
# environment
PROMPT_MULTILINE_INDICATOR = [90m∙[0m 
# environment
MANDATORY_PATH = /usr/share/gconf/pop.mandatory.path
# environment
LC_NAME = en_US.UTF-8
# environment
LC_NUMERIC = en_US.UTF-8
# default
LINK.S = $(CC) $(ASFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_MACH)
# environment
WINDOWID = 8388619
# environment
WINDOWPATH = 2
# environment
NVIM_LOG_FILE = /home/aj/.local/state/nvim/log
# default
LINK.s = $(CC) $(ASFLAGS) $(LDFLAGS) $(TARGET_MACH)
# environment
NVM_DIR = /home/aj/.nvm
# environment
LC_ADDRESS = en_US.UTF-8
# environment
LIBVIRT_DEFAULT_URI = qemu:///system
# default
MAKE_COMMAND := /usr/bin/make
# environment
QT_ACCESSIBILITY = 1
# automatic
@D = $(patsubst %/,%,$(dir $@))
# environment
MYVIMRC = /home/aj/.config/nvim/init.lua
# default
COFLAGS = 
# environment
TRANSIENT_PROMPT_COMMAND_RIGHT = 
# default
COMPILE.mod = $(M2C) $(M2FLAGS) $(MODFLAGS) $(TARGET_ARCH)
# default
.VARIABLES := 
# environment
PWD = /home/aj/repos/makevizzy
# automatic
%D = $(patsubst %/,%,$(dir $%))
# environment
PROMPT_INDICATOR_VI_INSERT = : 
# environment
XDG_DATA_DIRS = /usr/share/pop:/usr/share/gnome:/home/aj/.local/share/flatpak/exports/share:/var/lib/flatpak/exports/share:/usr/local/share/:/usr/share/:/var/lib/snapd/desktop
# default
LINK.o = $(CC) $(LDFLAGS) $(TARGET_ARCH)
# environment
OLDPWD = /home/aj/repos
# default
TEXI2DVI = texi2dvi
# automatic
^D = $(patsubst %/,%,$(dir $^))
# automatic
%F = $(notdir $%)
# environment
NVM_INC = /home/aj/.nvm/versions/node/v22.12.0/include/node
# default
LEX.l = $(LEX) $(LFLAGS) -t
# environment
LANG = C
# environment
XAUTHORITY = /run/user/1000/gdm/Xauthority
# default
.LOADED := 
# default
.INCLUDE_DIRS = /usr/local/include /usr/include /usr/include
# default
COMPILE.c = $(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c
# makefile
MAKEFLAGS = p
# default
LINK.f = $(FC) $(FFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# default
TANGLE = tangle
# makefile
CURDIR := /home/aj/repos/makevizzy
# default
PREPROCESS.F = $(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -F
# environment
LESSOPEN = | /usr/bin/lesspipe %s
# automatic
*D = $(patsubst %/,%,$(dir $*))
# environment
MFLAGS = -p
# environment
SSH_AUTH_SOCK = /run/user/1000/keyring/ssh
# default
.SHELLFLAGS := -c
# default
M2C = m2c
# default
COMPILE.p = $(PC) $(PFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c
# environment
NVM_BIN = /home/aj/.nvm/versions/node/v22.12.0/bin
# default
COMPILE.cpp = $(COMPILE.cc)
# default
TEX = tex
# environment
XDG_CONFIG_DIRS = /etc/xdg/xdg-pop:/etc/xdg
# automatic
+D = $(patsubst %/,%,$(dir $+))
# environment
XDG_SESSION_DESKTOP = pop
# environment
PROMPT_INDICATOR = 
# default
F77FLAGS = $(FFLAGS)
# makefile (from 'Makefile', line 1)
MAKEFILE_LIST := Makefile
# environment
GIT_EXTERNAL_DIFF = difft
# automatic
@F = $(notdir $@)
# environment
SDKMAN_DIR = /home/aj/.sdkman
# environment
XDG_SESSION_TYPE = x11
# environment
TMUX = /tmp/tmux-1000/default,16560,0
# automatic
?D = $(patsubst %/,%,$(dir $?))
# default
COMPILE.def = $(M2C) $(M2FLAGS) $(DEFFLAGS) $(TARGET_ARCH)
# default
CTANGLE = ctangle
# environment
SESSION_MANAGER = local/pop-os:@/tmp/.ICE-unix/14399,unix/pop-os:/tmp/.ICE-unix/14399
# automatic
*F = $(notdir $*)
# environment
MANPATH = /home/aj/.local/kitty.app/share/man:
# environment
DBUS_SESSION_BUS_ADDRESS = unix:path=/run/user/1000/bus
# automatic
<D = $(patsubst %/,%,$(dir $<))
# default
COMPILE.C = $(COMPILE.cc)
# default
YACC.m = $(YACC) $(YFLAGS)
# default
LINK.C = $(LINK.cc)
# default
MAKE_HOST := x86_64-pc-linux-gnu
# default
LINK.c = $(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# environment
GNOME_DESKTOP_SESSION_ID = this-is-deprecated
# makefile
SHELL = /bin/sh
# environment
PROMPT_INDICATOR_VI_NORMAL = > 
# environment
KITTY_PUBLIC_KEY = 1:ST#S7A3lf>f9A(wQiiBbgO!Hcf_(j%M0g#C4W(>F
# environment
XMODIFIERS = @im=ibus
# environment
DOTNET_BUNDLE_EXTRACT_BASE_DIR = /home/aj/.cache/dotnet_bundle_extract
# default
LINK.F = $(FC) $(FFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# environment
SHLVL = 3
# environment
TERMINFO = /home/aj/.local/kitty.app/lib/kitty/terminfo
# environment
NU_VERSION = 0.105.1
# environment
TRANSIENT_PROMPT_MULTILINE_INDICATOR = 
# environment
MAKELEVEL := 0
# environment
LAST_EXIT_CODE = 2
# default
MAKE = $(MAKE_COMMAND)
# default
FC = f77
# environment
PATH = /home/aj/.local/share/nvim/mason/bin:/home/aj/.elan/bin:/home/aj/.nvm/versions/node/v22.12.0/bin:/home/aj/.local/bin:/home/aj/.deno/bin:/home/aj/.sdkman/candidates/java/current/bin:/home/aj/.sdkman/candidates/gradle/current/bin:/home/aj/.juliaup/bin:/home/aj/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/snap/bin:/home/aj/.dotnet/tools:/home/aj/.zvm/bin:/home/aj/.zvm/self/:/usr/local/go/bin:/home/aj/.local/share/JetBrains/Toolbox/scripts:/home/aj/.nimble/bin:/home/aj/go/bin:/home/aj/.ghcup/hls/2.10.0.0/bin:/home/aj/.elan/bin:/home/aj/.julia/bin:/home/linuxbrew/.linuxbrew/bin:/home/aj/.zvm/bin:/home/aj/.zvm/self:/home/aj/.nimble/bin:/home/aj/go/bin:/home/aj/.ghcup/hls/2.10.0.0/bin:/home/aj/.elan/bin:/home/aj/.julia/bin:/home/linuxbrew/.linuxbrew/bin:/home/aj/.zvm/bin:/home/aj/.zvm/self:/home/aj/.nimble/bin:/home/aj/go/bin:/home/aj/.ghcup/hls/2.10.0.0/bin:/home/aj/.elan/bin:/home/aj/.julia/bin:/home/linuxbrew/.linuxbrew/bin:/home/aj/.zvm/bin:/home/aj/.zvm/self
# default
LINT = lint
# default
PC = pc
# default
MAKEFILES := 
# environment
LANGUAGE = en_GB:en
# environment
LC_MONETARY = en_US.UTF-8
# automatic
^F = $(notdir $^)
# default
LEX.m = $(LEX) $(LFLAGS) -t
# environment
LC_TIME = en_US.UTF-8
# default
.LIBPATTERNS = lib%.so lib%.a
# environment
SDKMAN_CANDIDATES_DIR = /home/aj/.sdkman/candidates
# environment
GRADLE_HOME = /home/aj/.sdkman/candidates/gradle/current
# environment
INVOCATION_ID = 3bc7ae3c9f874299a9f6e771b4eddf88
# default
CPP = $(CC) -E
# default
LINK.cc = $(CXX) $(CXXFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# environment
USERNAME = aj
# environment
TERM_PROGRAM = tmux
# default
CHECKOUT,v = +$(if $(wildcard $@),,$(CO) $(COFLAGS) $< $@)
# default
COMPILE.f = $(FC) $(FFLAGS) $(TARGET_ARCH) -c
# default
COMPILE.r = $(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -c
# environment
LC_TELEPHONE = en_US.UTF-8
# environment
LESSCLOSE = /usr/bin/lesspipe %s %s
# default
COMPILE.S = $(CC) $(ASFLAGS) $(CPPFLAGS) $(TARGET_MACH) -c
# automatic
?F = $(notdir $?)
# default
GET = get
# default
LINK.r = $(FC) $(FFLAGS) $(RFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# environment
GTK_IM_MODULE = ibus
# makefile (from 'Makefile', line 1)
ALL := output.dot output.pdf
# environment
XDG_CURRENT_DESKTOP = pop:GNOME
# environment
LS_COLORS = 
# automatic
+F = $(notdir $+)
# environment
DESKTOP_SESSION = pop
# default
MAKEINFO = makeinfo
# 'override' directive
GNUMAKEFLAGS := 
# default
PREPROCESS.r = $(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -F
# default
LINK.m = $(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# environment
LOGNAME = aj
# default
LINK.p = $(PC) $(PFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# environment
ZVM_INSTALL = /home/aj/.zvm/self
# default
YACC = yacc
# makefile
.DEFAULT_GOAL := all
# environment
SYSTEMD_EXEC_PID = 14569
# default
RM = rm -f
# environment
KITTY_WINDOW_ID = 1
# environment
EDITOR = nvim
# environment
DISPLAY = :1
# environment
GTK_MODULES = gail:atk-bridge:appmenu-gtk-module
# environment
NUSHELL_LAST_SHELL = 0
# environment
USER = aj
# default
WEAVE = weave
# environment
DEFAULTS_PATH = /usr/share/gconf/pop.default.path
# default
MAKE_VERSION := 4.3
# default
F77 = $(FC)
# environment
MANAGERPID = 13957
# environment
LC_MEASUREMENT = en_US.UTF-8
# environment
STARSHIP_SHELL = nu
# environment
GIO_LAUNCHED_DESKTOP_FILE_PID = 16290
# environment
KITTY_PID = 16290
# default
CWEAVE = cweave
# environment
_ = /home/aj/.cargo/bin/nu
# default
YACC.y = $(YACC) $(YFLAGS)
# environment
LC_PAPER = en_US.UTF-8
# default
LINK.cpp = $(LINK.cc)
# default
CO = co
# environment
XDG_RUNTIME_DIR = /run/user/1000
# environment
GPG_AGENT_INFO = /run/user/1000/gnupg/S.gpg-agent:0:1
# environment
COLORTERM = truecolor
# default
OUTPUT_OPTION = -o $@
# default
COMPILE.s = $(AS) $(ASFLAGS) $(TARGET_MACH)
# environment
NVM_CD_FLAGS = 
# environment
JOURNAL_STREAM = 9:72226
# default
MAKE_TERMERR := /dev/pts/2
# environment
XDG_SESSION_CLASS = user
# environment
TMUX_PLUGIN_MANAGER_PATH = /home/aj/.tmux/plugins/
# environment
SDKMAN_CANDIDATES_API = https://api.sdkman.io/2
# environment
HOME = /home/aj
# environment
QT_IM_MODULE = ibus
# default
LEX = lex
# environment
TERM = xterm-256color
# default
LINT.c = $(LINT) $(LINTFLAGS) $(CPPFLAGS) $(TARGET_ARCH)
# default
COMPILE.F = $(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c
# environment
PAPERSIZE = letter
# default
.RECIPEPREFIX := 
# automatic
<F = $(notdir $<)
# default
SUFFIXES := .out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S .mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo .w .ch .web .sh .elc .el
# default
LD = ld
# default
.FEATURES := target-specific order-only second-expansion else-if shortest-stem undefine oneshell nocomment grouped-target extra-prereqs archives jobserver output-sync check-symlink load
# default
CXX = g++
# default
CC = cc
# environment
KITTY_INSTALLATION_DIR = /home/aj/.local/kitty.app/lib/kitty
# environment
KITTY_LISTEN_ON = unix:/tmp/kitty-16290
# environment
XDG_MENU_PREFIX = gnome-
# environment
TERM_PROGRAM_VERSION = 3.2a
# default
COMPILE.cc = $(CXX) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c
# environment
GDMSESSION = pop
# environment
LC_IDENTIFICATION = en_US.UTF-8
# variable set hash-table stats:
# Load=201/1024=20%, Rehash=0, Collisions=41/232=18%
# Pattern-specific Variable Values
# No pattern-specific variable values.
# Directories
# RCS: could not be stat'd.
# SCCS: could not be stat'd.
# src (device 0, inode 0): No files, 19 impossibilities.
# . (device 66311, inode 21497469): 14 files, 38 impossibilities.
# src/RCS: could not be stat'd.
# src/SCCS: could not be stat'd.
# 14 files, 57 impossibilities in 6 directories.
# Implicit Rules
%.out:
%.a:
%.ln:
%.o:
%: %.o
#  recipe to execute (built-in):
	$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.c:
%: %.c
#  recipe to execute (built-in):
	$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.ln: %.c
#  recipe to execute (built-in):
	$(LINT.c) -C$* $<
%.o: %.c
#  recipe to execute (built-in):
	$(COMPILE.c) $(OUTPUT_OPTION) $<
%.cc:
%: %.cc
#  recipe to execute (built-in):
	$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.cc
#  recipe to execute (built-in):
	$(COMPILE.cc) $(OUTPUT_OPTION) $<
%.C:
%: %.C
#  recipe to execute (built-in):
	$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.C
#  recipe to execute (built-in):
	$(COMPILE.C) $(OUTPUT_OPTION) $<
%.cpp:
%: %.cpp
#  recipe to execute (built-in):
	$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.cpp
#  recipe to execute (built-in):
	$(COMPILE.cpp) $(OUTPUT_OPTION) $<
%.p:
%: %.p
#  recipe to execute (built-in):
	$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.p
#  recipe to execute (built-in):
	$(COMPILE.p) $(OUTPUT_OPTION) $<
%.f:
%: %.f
#  recipe to execute (built-in):
	$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.f
#  recipe to execute (built-in):
	$(COMPILE.f) $(OUTPUT_OPTION) $<
%.F:
%: %.F
#  recipe to execute (built-in):
	$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.F
#  recipe to execute (built-in):
	$(COMPILE.F) $(OUTPUT_OPTION) $<
%.f: %.F
#  recipe to execute (built-in):
	$(PREPROCESS.F) $(OUTPUT_OPTION) $<
%.m:
%: %.m
#  recipe to execute (built-in):
	$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.m
#  recipe to execute (built-in):
	$(COMPILE.m) $(OUTPUT_OPTION) $<
%.r:
%: %.r
#  recipe to execute (built-in):
	$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.r
#  recipe to execute (built-in):
	$(COMPILE.r) $(OUTPUT_OPTION) $<
%.f: %.r
#  recipe to execute (built-in):
	$(PREPROCESS.r) $(OUTPUT_OPTION) $<
%.y:
%.ln: %.y
#  recipe to execute (built-in):
	$(YACC.y) $< 
	 $(LINT.c) -C$* y.tab.c 
	 $(RM) y.tab.c
%.c: %.y
#  recipe to execute (built-in):
	$(YACC.y) $< 
	 mv -f y.tab.c $@
%.l:
%.ln: %.l
#  recipe to execute (built-in):
	@$(RM) $*.c
	 $(LEX.l) $< > $*.c
	$(LINT.c) -i $*.c -o $@
	 $(RM) $*.c
%.c: %.l
#  recipe to execute (built-in):
	@$(RM) $@ 
	 $(LEX.l) $< > $@
%.r: %.l
#  recipe to execute (built-in):
	$(LEX.l) $< > $@ 
	 mv -f lex.yy.r $@
%.ym:
%.m: %.ym
#  recipe to execute (built-in):
	$(YACC.m) $< 
	 mv -f y.tab.c $@
%.yl:
%.s:
%: %.s
#  recipe to execute (built-in):
	$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.s
#  recipe to execute (built-in):
	$(COMPILE.s) -o $@ $<
%.S:
%: %.S
#  recipe to execute (built-in):
	$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@
%.o: %.S
#  recipe to execute (built-in):
	$(COMPILE.S) -o $@ $<
%.s: %.S
#  recipe to execute (built-in):
	$(PREPROCESS.S) $< > $@
%.mod:
%: %.mod
#  recipe to execute (built-in):
	$(COMPILE.mod) -o $@ -e $@ $^
%.o: %.mod
#  recipe to execute (built-in):
	$(COMPILE.mod) -o $@ $<
%.sym:
%.def:
%.sym: %.def
#  recipe to execute (built-in):
	$(COMPILE.def) -o $@ $<
%.h:
%.info:
%.dvi:
%.tex:
%.dvi: %.tex
#  recipe to execute (built-in):
	$(TEX) $<
%.texinfo:
%.info: %.texinfo
#  recipe to execute (built-in):
	$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@
%.dvi: %.texinfo
#  recipe to execute (built-in):
	$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<
%.texi:
%.info: %.texi
#  recipe to execute (built-in):
	$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@
%.dvi: %.texi
#  recipe to execute (built-in):
	$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<
%.txinfo:
%.info: %.txinfo
#  recipe to execute (built-in):
	$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@
%.dvi: %.txinfo
#  recipe to execute (built-in):
	$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<
%.w:
%.c: %.w
#  recipe to execute (built-in):
	$(CTANGLE) $< - $@
%.tex: %.w
#  recipe to execute (built-in):
	$(CWEAVE) $< - $@
%.ch:
%.web:
%.p: %.web
#  recipe to execute (built-in):
	$(TANGLE) $<
%.tex: %.web
#  recipe to execute (built-in):
	$(WEAVE) $<
%.sh:
%: %.sh
#  recipe to execute (built-in):
	cat $< >$@ 
	 chmod a+x $@
%.elc:
%.el:
(%): %
#  recipe to execute (built-in):
	$(AR) $(ARFLAGS) $@ $<
%.out: %
#  recipe to execute (built-in):
	@rm -f $@ 
	 cp $< $@
%.c: %.w %.ch
#  recipe to execute (built-in):
	$(CTANGLE) $^ $@
%.tex: %.w %.ch
#  recipe to execute (built-in):
	$(CWEAVE) $^ $@
%:: %,v
#  recipe to execute (built-in):
	$(CHECKOUT,v)
%:: RCS/%,v
#  recipe to execute (built-in):
	$(CHECKOUT,v)
%:: RCS/%
#  recipe to execute (built-in):
	$(CHECKOUT,v)
%:: s.%
#  recipe to execute (built-in):
	$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<
%:: SCCS/s.%
#  recipe to execute (built-in):
	$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<
# 92 implicit rules, 5 (5.4%) terminal.
# Files
# Not a target:
.cpp:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.c.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.c) $(OUTPUT_OPTION) $<
output.dot: Makefile src/main.rs
#  Implicit rule search has not been done.
#  Last modified 2026-04-17 21:01:25.388212961
#  File has not been updated.
#  recipe to execute (from 'Makefile', line 9):
	LANG=C make -p | makevizzy o> output.dot
# Not a target:
.h:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.sh:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	cat $< >$@ 
	 chmod a+x $@
# Not a target:
.ch:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.r.f:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(PREPROCESS.r) $(OUTPUT_OPTION) $<
# Not a target:
.dvi:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.def.sym:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.def) -o $@ $<
# Not a target:
.m.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.m) $(OUTPUT_OPTION) $<
# Not a target:
.lm.m:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	@$(RM) $@ 
	 $(LEX.m) $< > $@
# Not a target:
.p.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.p) $(OUTPUT_OPTION) $<
# Not a target:
.texinfo:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.ln:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.C:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.web:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.elc:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.y.ln:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(YACC.y) $< 
	 $(LINT.c) -C$* y.tab.c 
	 $(RM) y.tab.c
# Not a target:
.l.c:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	@$(RM) $@ 
	 $(LEX.l) $< > $@
# Not a target:
Makefile:
#  Implicit rule search has been done.
#  Last modified 2026-04-17 20:59:19.65082867
#  File has been updated.
#  Successfully updated.
# Not a target:
.sym:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.r.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.r) $(OUTPUT_OPTION) $<
# Not a target:
.mod:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.mod) -o $@ -e $@ $^
# Not a target:
.def:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.S:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.texi.dvi:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<
# Not a target:
.txinfo.dvi:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<
# Not a target:
.y.c:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(YACC.y) $< 
	 mv -f y.tab.c $@
clean:
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (from 'Makefile', line 12):
	rm -f $(ALL)
# Not a target:
.cpp.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.cpp) $(OUTPUT_OPTION) $<
# Not a target:
.el:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.cc:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.tex:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.m:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.F:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.web.tex:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(WEAVE) $<
# Not a target:
.texinfo.info:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@
# Not a target:
.ym.m:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(YACC.m) $< 
	 mv -f y.tab.c $@
# Not a target:
.l:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.f:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.texi:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.DEFAULT:
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.r:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.a:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
all: output.pdf
#  Implicit rule search has been done.
#  File does not exist.
#  File has not been updated.
# Not a target:
.w.tex:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(CWEAVE) $< - $@
# Not a target:
.s.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.s) -o $@ $<
# Not a target:
.txinfo:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.c.ln:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINT.c) -C$* $<
# Not a target:
.tex.dvi:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(TEX) $<
# Not a target:
.info:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.out:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.texinfo.dvi:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<
# Not a target:
.F.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.F) $(OUTPUT_OPTION) $<
# Not a target:
.yl:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.s:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.S.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.S) -o $@ $<
# Not a target:
.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.C.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.C) $(OUTPUT_OPTION) $<
# Not a target:
.c:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@
# Not a target:
.txinfo.info:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@
# Not a target:
.texi.info:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@
# Not a target:
.y:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.l.r:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LEX.l) $< > $@ 
	 mv -f lex.yy.r $@
# Not a target:
.p:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@
output.pdf: output.dot
#  Implicit rule search has not been done.
#  Last modified 2026-04-17 21:01:25.410121035
#  File has not been updated.
#  recipe to execute (from 'Makefile', line 6):
	cat $< | dot -Tpdf -o $@
# Not a target:
.l.ln:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	@$(RM) $*.c
	 $(LEX.l) $< > $*.c
	$(LINT.c) -i $*.c -o $@
	 $(RM) $*.c
# Not a target:
.w:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.SUFFIXES: .out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S .mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo .w .ch .web .sh .elc .el
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.mod.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.mod) -o $@ $<
# Not a target:
.web.p:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(TANGLE) $<
# Not a target:
src/main.rs:
#  Implicit rule search has been done.
#  File does not exist.
#  File has not been updated.
# Not a target:
.S.s:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(PREPROCESS.S) $< > $@
# Not a target:
.f.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.f) $(OUTPUT_OPTION) $<
# Not a target:
.ym:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
# Not a target:
.cc.o:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(COMPILE.cc) $(OUTPUT_OPTION) $<
# Not a target:
.F.f:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(PREPROCESS.F) $(OUTPUT_OPTION) $<
# Not a target:
.w.c:
#  Builtin rule
#  Implicit rule search has not been done.
#  Modification time never checked.
#  File has not been updated.
#  recipe to execute (built-in):
	$(CTANGLE) $< - $@
# files hash-table stats:
# Load=78/1024=8%, Rehash=0, Collisions=152/1796=8%
# VPATH Search Paths
# No 'vpath' search paths.
# No general ('VPATH' variable) search path.
# strcache buffers: 1 (0) / strings = 513 / storage = 6108 B / avg = 11 B
# current buf: size = 8162 B / used = 6108 B / count = 513 / avg = 11 B
# strcache performance: lookups = 834 / hit rate = 38%
# hash-table stats:
# Load=513/8192=6%, Rehash=0, Collisions=55/834=7%
# Finished Make data base on Fri Apr 17 22:06:28 2026"
        .split('\n')
        .map(|s| s.to_string() + "\n");

    let result = crate::parse_make_p(&mut makefile);

    assert!(result.is_ok());

    let expected = make_targets(&[
        (".cpp", &["\n"]),
        (".c.o", &["\n"]),
        ("output.dot", &["Makefile", "src/main.rs\n"]),
        (".h", &["\n"]),
        (".sh", &["\n"]),
        (".ch", &["\n"]),
        (".r.f", &["\n"]),
        (".dvi", &["\n"]),
        (".def.sym", &["\n"]),
        (".m.o", &["\n"]),
        (".lm.m", &["\n"]),
        (".p.o", &["\n"]),
        (".texinfo", &["\n"]),
        (".ln", &["\n"]),
        (".C", &["\n"]),
        (".web", &["\n"]),
        (".elc", &["\n"]),
        (".y.ln", &["\n"]),
        (".l.c", &["\n"]),
        ("Makefile", &["\n"]),
        (".sym", &["\n"]),
        (".r.o", &["\n"]),
        (".mod", &["\n"]),
        (".def", &["\n"]),
        (".S", &["\n"]),
        (".texi.dvi", &["\n"]),
        (".txinfo.dvi", &["\n"]),
        (".y.c", &["\n"]),
        ("clean", &["\n"]),
        (".cpp.o", &["\n"]),
        (".el", &["\n"]),
        (".cc", &["\n"]),
        (".tex", &["\n"]),
        (".m", &["\n"]),
        (".F", &["\n"]),
        (".web.tex", &["\n"]),
        (".texinfo.info", &["\n"]),
        (".ym.m", &["\n"]),
        (".l", &["\n"]),
        (".f", &["\n"]),
        (".texi", &["\n"]),
        (".DEFAULT", &["\n"]),
        (".r", &["\n"]),
        (".a", &["\n"]),
        ("all", &["output.pdf\n"]),
        (".w.tex", &["\n"]),
        (".s.o", &["\n"]),
        (".txinfo", &["\n"]),
        (".c.ln", &["\n"]),
        (".tex.dvi", &["\n"]),
        (".info", &["\n"]),
        (".out", &["\n"]),
        (".texinfo.dvi", &["\n"]),
        (".F.o", &["\n"]),
        (".yl", &["\n"]),
        (".s", &["\n"]),
        (".S.o", &["\n"]),
        (".o", &["\n"]),
        (".C.o", &["\n"]),
        (".c", &["\n"]),
        (".txinfo.info", &["\n"]),
        (".texi.info", &["\n"]),
        (".y", &["\n"]),
        (".l.r", &["\n"]),
        (".p", &["\n"]),
        ("output.pdf", &["output.dot\n"]),
        (".l.ln", &["\n"]),
        (".w", &["\n"]),
        (
            ".SUFFIXES",
            &[
                ".out", ".a", ".ln", ".o", ".c", ".cc", ".C", ".cpp", ".p", ".f", ".F", ".m", ".r",
                ".y", ".l", ".ym", ".yl", ".s", ".S", ".mod", ".sym", ".def", ".h", ".info",
                ".dvi", ".tex", ".texinfo", ".texi", ".txinfo", ".w", ".ch", ".web", ".sh", ".elc",
                ".el\n",
            ],
        ),
        (".mod.o", &["\n"]),
        (".web.p", &["\n"]),
        ("src/main.rs", &["\n"]),
        (".S.s", &["\n"]),
        (".f.o", &["\n"]),
        (".ym", &["\n"]),
        (".cc.o", &["\n"]),
        (".F.f", &["\n"]),
        (".w.c", &["\n"]),
    ]);

    assert_eq!(expected, result.unwrap());
}
