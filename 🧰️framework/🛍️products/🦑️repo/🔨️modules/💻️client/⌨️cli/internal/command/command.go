// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package command provides the owned command schema, flag parser, help renderer, and dispatcher.

// #endregion 🧲️Header

package command

import (
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
	"time"
)

// #region 📜️Schema

type PositionalValidator func(*Command, []string) error

type Command struct {
	Use                string
	Short              string
	Long               string
	Example            string
	Version            string
	Aliases            []string
	Args               PositionalValidator
	Run                func(*Command, []string)
	RunE               func(*Command, []string) error
	SilenceUsage       bool
	SilenceErrors      bool
	DisableFlagParsing bool
	PersistentPreRunE  func(*Command, []string) error
	PreRunE            func(*Command, []string) error
	parent             *Command
	children           []*Command
	flags              *FlagSet
	persistentFlags    *FlagSet
	args               []string
	out                io.Writer
	err                io.Writer
	in                 io.Reader
	ctx                context.Context
}

type Flag struct {
	Name         string
	Shorthand    string
	Usage        string
	Changed      bool
	Value        Value
	value        any
	defaultValue any
	bound        any
}

type Value interface{ String() string }

type stringValue string

func (value stringValue) String() string { return string(value) }

type FlagSet struct {
	flags map[string]*Flag
	short map[string]*Flag
}

// #endregion 📜️Schema

// #region 🏳️Flags

func newFlagSet() *FlagSet { return &FlagSet{flags: map[string]*Flag{}, short: map[string]*Flag{}} }

func (set *FlagSet) add(name, shorthand, usage string, value, bound any) *Flag {
	flag := &Flag{Name: name, Shorthand: shorthand, Usage: usage, value: value, defaultValue: value, bound: bound}
	flag.sync()
	set.flags[name] = flag
	if shorthand != "" {
		set.short[shorthand] = flag
	}
	return flag
}

func (flag *Flag) sync() {
	switch value := flag.value.(type) {
	case string:
		flag.Value = stringValue(value)
		if target, ok := flag.bound.(*string); ok {
			*target = value
		}
	case bool:
		flag.Value = stringValue(strconv.FormatBool(value))
		if target, ok := flag.bound.(*bool); ok {
			*target = value
		}
	case int:
		flag.Value = stringValue(strconv.Itoa(value))
		if target, ok := flag.bound.(*int); ok {
			*target = value
		}
	case []string:
		flag.Value = stringValue(strings.Join(value, ","))
		if target, ok := flag.bound.(*[]string); ok {
			*target = append((*target)[:0], value...)
		}
	case []int:
		parts := make([]string, len(value))
		for index := range value {
			parts[index] = strconv.Itoa(value[index])
		}
		flag.Value = stringValue(strings.Join(parts, ","))
		if target, ok := flag.bound.(*[]int); ok {
			*target = append((*target)[:0], value...)
		}
	case time.Duration:
		flag.Value = stringValue(value.String())
		if target, ok := flag.bound.(*time.Duration); ok {
			*target = value
		}
	}
}

func (set *FlagSet) reset() {
	for _, flag := range set.flags {
		flag.Changed = false
		flag.value = flag.defaultValue
		flag.sync()
	}
}

func (set *FlagSet) String(name, value, usage string) *string {
	target := new(string)
	set.add(name, "", usage, value, target)
	return target
}

func (set *FlagSet) StringVar(target *string, name, value, usage string) {
	set.add(name, "", usage, value, target)
}

func (set *FlagSet) StringVarP(target *string, name, shorthand, value, usage string) {
	set.add(name, shorthand, usage, value, target)
}

func (set *FlagSet) Bool(name string, value bool, usage string) *bool {
	target := new(bool)
	set.add(name, "", usage, value, target)
	return target
}

func (set *FlagSet) BoolP(name, shorthand string, value bool, usage string) *bool {
	target := new(bool)
	set.add(name, shorthand, usage, value, target)
	return target
}

func (set *FlagSet) BoolVar(target *bool, name string, value bool, usage string) {
	set.add(name, "", usage, value, target)
}

func (set *FlagSet) Int(name string, value int, usage string) *int {
	target := new(int)
	set.add(name, "", usage, value, target)
	return target
}

func (set *FlagSet) IntSlice(name string, value []int, usage string) *[]int {
	target := new([]int)
	set.add(name, "", usage, append([]int(nil), value...), target)
	return target
}

func (set *FlagSet) DurationVar(target *time.Duration, name string, value time.Duration, usage string) {
	set.add(name, "", usage, value, target)
}

func (set *FlagSet) StringSlice(name string, value []string, usage string) *[]string {
	target := new([]string)
	set.add(name, "", usage, append([]string(nil), value...), target)
	return target
}

func (set *FlagSet) Lookup(name string) *Flag { return set.flags[name] }

func (set *FlagSet) Changed(name string) bool {
	flag := set.flags[name]
	return flag != nil && flag.Changed
}

func (set *FlagSet) Set(name, value string) error {
	flag, err := set.get(name)
	if err != nil {
		return err
	}
	return set.set(flag, value)
}

func (set *FlagSet) MarkHidden(name string) error {
	_, err := set.get(name)
	return err
}

func (set *FlagSet) GetString(name string) (string, error) {
	flag, err := set.get(name)
	if err != nil {
		return "", err
	}
	value, ok := flag.value.(string)
	if !ok {
		return "", fmt.Errorf("flag --%s is not a string", name)
	}
	return value, nil
}

func (set *FlagSet) GetBool(name string) (bool, error) {
	flag, err := set.get(name)
	if err != nil {
		return false, err
	}
	value, ok := flag.value.(bool)
	if !ok {
		return false, fmt.Errorf("flag --%s is not a boolean", name)
	}
	return value, nil
}

func (set *FlagSet) GetInt(name string) (int, error) {
	flag, err := set.get(name)
	if err != nil {
		return 0, err
	}
	value, ok := flag.value.(int)
	if !ok {
		return 0, fmt.Errorf("flag --%s is not an integer", name)
	}
	return value, nil
}

func (set *FlagSet) GetStringSlice(name string) ([]string, error) {
	flag, err := set.get(name)
	if err != nil {
		return nil, err
	}
	value, ok := flag.value.([]string)
	if !ok {
		return nil, fmt.Errorf("flag --%s is not a string list", name)
	}
	return append([]string(nil), value...), nil
}

func (set *FlagSet) GetIntSlice(name string) ([]int, error) {
	flag, err := set.get(name)
	if err != nil {
		return nil, err
	}
	value, ok := flag.value.([]int)
	if !ok {
		return nil, fmt.Errorf("flag --%s is not an integer list", name)
	}
	return append([]int(nil), value...), nil
}

func (set *FlagSet) get(name string) (*Flag, error) {
	flag := set.flags[name]
	if flag == nil {
		return nil, fmt.Errorf("unknown flag: --%s", name)
	}
	return flag, nil
}

func (set *FlagSet) set(flag *Flag, raw string) error {
	switch flag.value.(type) {
	case string:
		flag.value = raw
	case bool:
		value, err := strconv.ParseBool(raw)
		if err != nil {
			return fmt.Errorf("invalid boolean for --%s: %s", flag.Name, raw)
		}
		flag.value = value
	case int:
		value, err := strconv.Atoi(raw)
		if err != nil {
			return fmt.Errorf("invalid integer for --%s: %s", flag.Name, raw)
		}
		flag.value = value
	case []string:
		if raw == "" {
			flag.value = []string{}
		} else {
			flag.value = strings.Split(raw, ",")
		}
	case []int:
		var values []int
		if raw != "" {
			for _, part := range strings.Split(raw, ",") {
				value, err := strconv.Atoi(part)
				if err != nil {
					return fmt.Errorf("invalid integer list for --%s: %s", flag.Name, raw)
				}
				values = append(values, value)
			}
		}
		flag.value = values
	case time.Duration:
		value, err := time.ParseDuration(raw)
		if err != nil {
			return fmt.Errorf("invalid duration for --%s: %s", flag.Name, raw)
		}
		flag.value = value
	default:
		return fmt.Errorf("unsupported flag --%s", flag.Name)
	}
	flag.Changed = true
	flag.sync()
	return nil
}

// #endregion 🏳️Flags

// #region 🧭️CommandTree

func (command *Command) Flags() *FlagSet {
	if command.flags == nil {
		command.flags = newFlagSet()
	}
	return command.flags
}

func (command *Command) PersistentFlags() *FlagSet {
	if command.persistentFlags == nil {
		command.persistentFlags = newFlagSet()
	}
	return command.persistentFlags
}

func (command *Command) AddCommand(children ...*Command) {
	for _, child := range children {
		child.parent = command
		command.children = append(command.children, child)
	}
}

func (command *Command) Commands() []*Command { return append([]*Command(nil), command.children...) }
func (command *Command) Parent() *Command     { return command.parent }
func (command *Command) Root() *Command {
	root := command
	for root.parent != nil {
		root = root.parent
	}
	return root
}

func (command *Command) Name() string {
	name, _, _ := strings.Cut(command.Use, " ")
	return name
}

func (command *Command) Context() context.Context {
	if command.ctx != nil {
		return command.ctx
	}
	if command.parent != nil {
		return command.parent.Context()
	}
	return context.Background()
}

func (command *Command) SetContext(ctx context.Context) { command.ctx = ctx }
func (command *Command) SetArgs(args []string)          { command.args = append([]string(nil), args...) }
func (command *Command) SetOut(writer io.Writer)        { command.Root().out = writer }
func (command *Command) SetErr(writer io.Writer)        { command.Root().err = writer }
func (command *Command) SetIn(reader io.Reader)         { command.Root().in = reader }
func (command *Command) OutOrStdout() io.Writer {
	if command.Root().out != nil {
		return command.Root().out
	}
	return os.Stdout
}
func (command *Command) ErrOrStderr() io.Writer {
	if command.Root().err != nil {
		return command.Root().err
	}
	return os.Stderr
}

func (command *Command) InOrStdin() io.Reader {
	if command.Root().in != nil {
		return command.Root().in
	}
	return os.Stdin
}

func (command *Command) Println(values ...interface{}) {
	fmt.Fprintln(command.OutOrStdout(), values...)
}

func (command *Command) Help() error {
	writer := command.OutOrStdout()
	fmt.Fprintf(writer, "%s\n\nUsage:\n  %s", command.Short, command.Use)
	if len(command.children) > 0 {
		fmt.Fprintln(writer, "\n\nCommands:")
		for _, child := range command.children {
			fmt.Fprintf(writer, "  %-20s %s\n", child.Name(), child.Short)
		}
	}
	return nil
}

// #endregion 🧭️CommandTree

// #region ▶️Dispatch

var errHelp = errors.New("help requested")

func (command *Command) Execute() error {
	_, err := command.execute(command.args)
	return err
}

func (command *Command) ExecuteC() (*Command, error) { return command.execute(command.args) }

func (command *Command) execute(args []string) (*Command, error) {
	selected, positional, err := command.selectAndParse(args)
	if err != nil {
		if errors.Is(err, errHelp) {
			return selected, nil
		}
		return selected, err
	}
	if selected.Args != nil {
		if err := selected.Args(selected, positional); err != nil {
			return selected, err
		}
	}
	for _, ancestor := range selected.ancestry() {
		if ancestor.PersistentPreRunE != nil {
			if err := ancestor.PersistentPreRunE(selected, positional); err != nil {
				return selected, err
			}
		}
	}
	if selected.PreRunE != nil {
		if err := selected.PreRunE(selected, positional); err != nil {
			return selected, err
		}
	}
	if selected.RunE != nil {
		return selected, selected.RunE(selected, positional)
	}
	if selected.Run != nil {
		selected.Run(selected, positional)
		return selected, nil
	}
	if len(selected.children) > 0 {
		return selected, selected.Help()
	}
	return selected, nil
}

func (command *Command) selectAndParse(args []string) (*Command, []string, error) {
	selected := command
	positional := make([]string, 0, len(args))
	for index := 0; index < len(args); index++ {
		arg := args[index]
		if selected.DisableFlagParsing {
			return selected, append(positional, args[index:]...), nil
		}
		if arg == "--help" || arg == "-h" {
			if err := selected.Help(); err != nil {
				return selected, nil, err
			}
			return selected, nil, errHelp
		}
		if arg == "--" {
			positional = append(positional, args[index+1:]...)
			break
		}
		if strings.HasPrefix(arg, "-") {
			consumed, err := selected.parseFlag(args[index:])
			if err != nil {
				return selected, nil, err
			}
			index += consumed - 1
			continue
		}
		if len(positional) == 0 {
			if child := selected.child(arg); child != nil {
				selected = child
				continue
			}
		}
		positional = append(positional, arg)
	}
	return selected, positional, nil
}

func (command *Command) parseFlag(args []string) (int, error) {
	token := args[0]
	name := strings.TrimPrefix(token, "--")
	short := false
	if name == token {
		name = strings.TrimPrefix(token, "-")
		short = true
	}
	inline := ""
	if key, value, found := strings.Cut(name, "="); found {
		name, inline = key, value
	}
	flag := command.lookupFlag(name, short)
	if flag == nil {
		return 0, fmt.Errorf("unknown flag: %s", token)
	}
	if _, ok := flag.value.(bool); ok && inline == "" {
		return 1, command.ownerSet(flag, "true")
	}
	if inline != "" {
		return 1, command.ownerSet(flag, inline)
	}
	if len(args) < 2 {
		return 0, fmt.Errorf("flag needs an argument: %s", token)
	}
	return 2, command.ownerSet(flag, args[1])
}

func (command *Command) ownerSet(flag *Flag, value string) error {
	for current := command; current != nil; current = current.parent {
		if current.flags != nil && current.flags.flags[flag.Name] == flag {
			return current.flags.set(flag, value)
		}
		if current.persistentFlags != nil && current.persistentFlags.flags[flag.Name] == flag {
			return current.persistentFlags.set(flag, value)
		}
	}
	return errors.New("flag owner missing")
}

func (command *Command) lookupFlag(name string, short bool) *Flag {
	if command.flags != nil {
		if short {
			if flag := command.flags.short[name]; flag != nil {
				return flag
			}
		}
		if flag := command.flags.flags[name]; flag != nil {
			return flag
		}
	}
	for current := command; current != nil; current = current.parent {
		if current.persistentFlags == nil {
			continue
		}
		if short {
			if flag := current.persistentFlags.short[name]; flag != nil {
				return flag
			}
		}
		if flag := current.persistentFlags.flags[name]; flag != nil {
			return flag
		}
	}
	return nil
}

func (command *Command) child(name string) *Command {
	for _, child := range command.children {
		if child.Name() == name {
			return child
		}
		for _, alias := range child.Aliases {
			if alias == name {
				return child
			}
		}
	}
	return nil
}

func (command *Command) ancestry() []*Command {
	var reversed []*Command
	for current := command; current != nil; current = current.parent {
		reversed = append(reversed, current)
	}
	ancestors := make([]*Command, len(reversed))
	for index := range reversed {
		ancestors[len(reversed)-index-1] = reversed[index]
	}
	return ancestors
}

func (command *Command) resetFlagsRecursive() {
	if command.flags != nil {
		command.flags.reset()
	}
	if command.persistentFlags != nil {
		command.persistentFlags.reset()
	}
	for _, child := range command.children {
		child.resetFlagsRecursive()
	}
}

// #endregion ▶️Dispatch

// #region 📍️PositionalValidation

func NoArgs(_ *Command, args []string) error {
	if len(args) != 0 {
		return fmt.Errorf("accepts 0 arg(s), received %d", len(args))
	}
	return nil
}

func ExactArgs(count int) PositionalValidator {
	return func(_ *Command, args []string) error {
		if len(args) != count {
			return fmt.Errorf("accepts %d arg(s), received %d", count, len(args))
		}
		return nil
	}
}

func MaximumNArgs(count int) PositionalValidator {
	return func(_ *Command, args []string) error {
		if len(args) > count {
			return fmt.Errorf("accepts at most %d arg(s), received %d", count, len(args))
		}
		return nil
	}
}

func RangeArgs(minimum, maximum int) PositionalValidator {
	return func(_ *Command, args []string) error {
		if len(args) < minimum || len(args) > maximum {
			return fmt.Errorf("accepts between %d and %d arg(s), received %d", minimum, maximum, len(args))
		}
		return nil
	}
}

func ArbitraryArgs(_ *Command, _ []string) error { return nil }

// #endregion 📍️PositionalValidation
