from _n64romconvert_wrapper import determine_format as _determine_format
from enum import Enum, auto
from pathlib import Path

ROMTYPE_CONVERSIONS = {
    "ByteSwapped": "v64",
    "LittleEndian": "n64",
    "BigEndian": "z64",
}

class ConversionError(Exception):
    """Represents a ROM conversion error."""

class RomError(Exception):
    """Represents a ROM-related error."""

class RomType(Enum):
    """Represents a ROM Type (n64, z64, v64)."""
    ByteSwapped = auto()
    LittleEndian = auto()
    BigEndian = auto()

    def __str__(self) -> str:
        return ROMTYPE_CONVERSIONS[self.name]

    @staticmethod
    def from_str(s: str) -> 'RomType':
        match s.lower():
            case "z64":
                return RomType.BigEndian
            case "v64":
                return RomType.ByteSwapped
            case "n64":
                return RomType.LittleEndian
            case _:
                raise ValueError(f"{s.lower()} is not a valid ROM type!")

def determine_format(path: str | Path) -> RomType:
    try:
        p = (path.__str__() if isinstance(path, Path)
                else path)

        fmt = _determine_format(p)
        return RomType.from_str(fmt)
    except ValueError as err:
        raise RomError(err.__str__())

class Rom:
    def __init__(self,
                 path: str | Path,
                 rom_type: RomType | None = None,
                 ) -> None:
        if isinstance(path, str):
            self.path = Path(path)
        else:
            self.path = path 

        if not (p := self.path).exists():
            raise FileNotFoundError(f"could not find a ROM at {p}.")
        
        if not rom_type:
            self.rom_type = determine_format(path)
        else:
            self.rom_type = rom_type
