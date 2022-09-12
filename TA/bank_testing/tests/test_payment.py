from datetime import date

import pytest
from bank.card import CreditCard, DebitCard
from bank.order import LineItem, Order, OrderStatus
from bank.payment import pay_order


@pytest.fixture
def card() -> CreditCard:
    year = date.today().year + 2
    return CreditCard("1249190007575069", 0.0, 1000.0, 1, year)


class PaymentProcessorMock:
    def charge(self, card: CreditCard | DebitCard, amount: float) -> None:
        print(f"Charging card {card.card_number} for {amount}.")


def test_pay_order(card: CreditCard) -> None:
    order = Order()
    order.line_items.append(LineItem(name="Shoes", price=100_00, quantity=2))
    pay_order(order, PaymentProcessorMock(), card)
    assert order.status == OrderStatus.PAID


def test_pay_order_invalid(card: CreditCard) -> None:
    with pytest.raises(ValueError):
        order = Order()
        pay_order(order, PaymentProcessorMock(), card)
