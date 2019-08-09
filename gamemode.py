from cffi import FFI


class StateGetter:
    def __init__(self, library):
        self.library = library

    def pop(self):
        result = []

        if not self.library.pop_state():
            return None

        i = self.library.state_int()

        if self.library.state_success():
            result += [i, ]

        f = self.library.state_float()

        if self.library.state_success():
            result += [f, ]

        b = self.library.state_bool()

        if self.library.state_success():
            result += [b, ]

        s = self._state_string()

        if s is not None:
            result += [s, ]

        return result

    def pop_many(self, n):
        result = []

        for _ in range(0, n):
            result += self.pop()
        
        return result[::-1]

    def push(self, value):
        return self.push_many([value])

    def push_many(self, values):
        for value in values:
            self.library.start_create_state()

            if type(value) == int:
                self.library.state_set_int(value)

            elif type(value) == float:
                self.library.state_set_float(value)

            elif type(value) == bool:
                self.library.state_set_bool(value)

            elif type(value) == str:
                for c in value:
                    sg.library.state_add_char(ord(c))
            
            else:
                raise TypeError(f'Invalid type for state: {type(value)}, excepted int/float/bool/str.')

            self.library.state_end()

    def _state_string(self):
        result = None

        while not self.library.state_string_end():
            if result is None:
                result = ''

            c = self.library.state_string()

            if c != -1:
                result += chr(c)
            else:
                if len(result) == 0:
                    return None

                break

            if len(result) == 0:
                return None

        return result


def get_lib():
    from platform import system

    ffi = FFI()
    lib = None

    if 'Windows' in system():
        lib = ffi.dlopen('gmlib.dll')
    else:
        lib = ffi.dlopen('gmlib.so')

    ffi.cdef('bool start_create_state();')
    ffi.cdef('bool state_set_int(long long);')
    ffi.cdef('bool state_set_float(double);')
    ffi.cdef('bool state_set_bool(bool);')
    ffi.cdef('bool state_add_char(int);')
    ffi.cdef('bool state_end();')

    ffi.cdef('bool pop_state();')
    ffi.cdef('bool state_success();')
    ffi.cdef('long long state_int();')
    ffi.cdef('double state_float();')
    ffi.cdef('bool state_bool();')
    ffi.cdef('int state_string();')
    ffi.cdef('bool state_string_end();')

    ffi.cdef('void start();')
    lib.start()

    return lib


lib = get_lib()
sg = StateGetter(lib)

sg.push_many([10, True, "Hello, World!"])
print(sg.pop_many(4))
