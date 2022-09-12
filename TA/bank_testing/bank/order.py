from dataclasses import dataclass, field
from enum import Enum


class OrderStatus(Enum):
    OPEN = "open"
    PAID = "paid"
    CANCELLED = "cancelled"
    PENDING = "pending"


@dataclass
class LineItem:
    name: str
    quantity: int
    price: float

    @property
    def total(self):
        return self.quantity * self.price


@dataclass
class Order:
    line_items: list[LineItem] = field(default_factory=list)
    status: OrderStatus = OrderStatus.OPEN

    @property
    def total(self) -> float:
        return sum([line_item.total for line_item in self.line_items])

    def pay(self) -> None:
        self.status = OrderStatus.PAID

