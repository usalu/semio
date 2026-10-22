// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚩️flag carries the flag set of the cli command framework.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"
	strconv "strconv"
	strings "strings"
	time "time"
)

// #region 🎮️Command Framework

type Flag struct {
	Name         string
	Shorthand    string
	Usage        string
	Changed      bool
	Single       bool
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
	owner *Command
}

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

// 🔗️resolve finds a flag by name: the set's own flags first, then the persistent
// flags inherited from the owning command's ancestors (mirroring [Command.Flags]).
func (set *FlagSet) resolve(name string) *Flag {
	if flag := set.flags[name]; flag != nil {
		return flag
	}
	if set.owner == nil {
		return nil
	}
	return set.owner.lookupFlag(name, false)
}

func (set *FlagSet) Lookup(name string) *Flag { return set.resolve(name) }

func (set *FlagSet) Changed(name string) bool {
	flag := set.resolve(name)
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
	flag := set.resolve(name)
	if flag == nil {
		return nil, fmt.Errorf("unknown flag: --%s", name)
	}
	return flag, nil
}

func (set *FlagSet) set(flag *Flag, raw string) error {
 if flag.Single && flag.Changed { return fmt.Errorf("flag --%s may only occur once",flag.Name) }
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

// #endregion 🎮️Command Framework
