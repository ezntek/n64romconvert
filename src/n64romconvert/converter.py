import _n64romconvert_wrapper as wrapper

from collections.abc import Callable
from pathlib import Path
from _n64romconvert_wrapper import RustPanicException
from . import *

_F = Callable[['Converter', Path], Rom]
def _check_convert_paths(f: _F):
    def wrapper(slf: 'Converter', target_path: Path):
        if not target_path.exists():
            raise FileNotFoundError(f"the ROM at path {target_path} does not exist.")
        elif not (parent := target_path.parent).exists():
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
        self.rom = rom
    
    @_check_convert_paths
    def to_byteswapped(self, target_path: Path) -> Rom:
        target_path_str: str = target_path.__str__()

        match self.rom.type:
            case RomType.ByteSwapped:
                raise ConversionError("Cannot convert between the same ROM types.")
            case RomType.BigEndian:
                _call_wrapper_func(wrapper.byte_swap, self.rom.path, target_path_str)
                return Rom(RomType.ByteSwapped, target_path_str)
            case RomType.LittleEndian:
                _call_wrapper_func(wrapper.byte_endian_swap, self.rom.path, target_path_str)
                return Rom(RomType.ByteSwapped, target_path_str)

    @_check_convert_paths
    def to_big_endian(self, target_path: Path) -> Rom:
        target_path_str: str = target_path.__str__()

        match self.rom.type:
            case RomType.BigEndian:
                raise ConversionError("Cannot convert between the same ROM types.")
            case RomType.LittleEndian:
                _call_wrapper_func(wrapper.endian_swap, self.rom.path, target_path_str)
                return Rom(RomType.LittleEndian, target_path_str)
            case RomType.ByteSwapped:
                _call_wrapper_func(wrapper.byte_swap, self.rom.path, target_path_str)
                return Rom(RomType.ByteSwapped, target_path_str)

    @_check_convert_paths
    def to_little_endian(self, target_path: Path) -> Rom:
        target_path_str: str = target_path.__str__()

        match self.rom.type:
            case RomType.LittleEndian:
                raise ConversionError("Cannot convert between the same ROM types.")
            case RomType.BigEndian:
                _call_wrapper_func(wrapper.endian_swap, self.rom.path, target_path_str)
                return Rom(RomType.LittleEndian, target_path_str)
            case RomType.ByteSwapped:
                _call_wrapper_func(wrapper.byte_endian_swap, self.rom.path, target_path_str)
                return Rom(RomType.ByteSwapped, target_path_str)
