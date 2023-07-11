from dataclasses import dataclass
from enum import Enum, auto

ROMTYPE_CONVERSIONS = {
    "ByteSwapped": "v64",
    "LittleEndian": "n64",
    "BigEndian": "z64",
}

class ConversionError(Exception):
    """Represents a ROM conversion error."""

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

@dataclass(frozen=True)
class Rom:
    type: RomType
    path: str

# unexpose via deletion
del ROMTYPE_CONVERSIONS
