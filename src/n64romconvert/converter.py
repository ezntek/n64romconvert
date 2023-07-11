import _n64romconvert_wrapper as wrapper

from collections.abc import Callable
from pathlib import Path
from _n64romconvert_wrapper import RustPanicException
from . import *

_F = Callable[['Converter', str], Rom]
def _convert(f: _F):
    def wrapper(slf: 'Converter', target_path: str):
        if not (parent := Path(target_path).parent).exists():
            raise FileNotFoundError(f"parent path of {target_path} ({parent}) does not exist.")

        f(slf, target_path)
    return wrapper

def _call_wrapper_func(f: Callable, *args): # type: ignore
    try:
        f(*args) # type: ignore
    except RustPanicException as exc:
        raise ConversionError(f"The internal rust code panicked: {exc.__str__()}")

class Converter:
    def __init__(self, rom: Rom) -> None:
        if not rom.path.exists():
            raise FileNotFoundError(f"ROM at path {rom.path} does not exist.")
        self.rom = rom
    
    @_convert
    def to_byteswapped(self, target_path: str) -> Rom:
        source_path = str(self.rom.path)

        match self.rom.rom_type:
            case RomType.ByteSwapped:
                raise ConversionError("Cannot convert between the same ROM types.")
            case RomType.BigEndian:
                _call_wrapper_func(wrapper.byte_swap, source_path, target_path)
                return Rom(target_path, RomType.BigEndian)
            case RomType.LittleEndian:
                _call_wrapper_func(wrapper.byte_endian_swap, source_path, target_path)
                return Rom(target_path, RomType.LittleEndian)

    @_convert
    def to_big_endian(self, target_path: str) -> Rom:
        source_path = str(self.rom.path)

        match self.rom.rom_type:
            case RomType.BigEndian:
                raise ConversionError("Cannot convert between the same ROM types.")
            case RomType.LittleEndian:
                _call_wrapper_func(wrapper.endian_swap, source_path, target_path)
                return Rom(target_path, RomType.LittleEndian)
            case RomType.ByteSwapped:
                _call_wrapper_func(wrapper.byte_swap, source_path, target_path)
                return Rom(target_path, RomType.ByteSwapped)

    @_convert
    def to_little_endian(self, target_path: str) -> Rom:
        source_path = str(self.rom.path)

        match self.rom.rom_type:
            case RomType.LittleEndian:
                raise ConversionError("Cannot convert between the same ROM types.")
            case RomType.BigEndian:
                _call_wrapper_func(wrapper.endian_swap, source_path, target_path)
                return Rom(target_path, RomType.LittleEndian)
            case RomType.ByteSwapped:
                _call_wrapper_func(wrapper.byte_endian_swap, source_path, target_path)
                return Rom(target_path, RomType.ByteSwapped)
