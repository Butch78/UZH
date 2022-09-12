from bank.order import LineItem, Order, OrderStatus


def test_empty_order() -> None:
    order = Order()
    assert order.total == 0.0


def test_order_total() -> None:
    order = Order()
    order.line_items.append(LineItem("T-Shirt", 1, 10.0))
    order.line_items.append(LineItem("T-Shirt", 1, 10.0))
    assert order.total == 20.0


def test_order_status() -> None:
    order = Order()
    order.pay()

    assert order.status == OrderStatus.PAID


