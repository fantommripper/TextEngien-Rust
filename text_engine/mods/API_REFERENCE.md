# Text Engine - Python API Reference

## Система управления окнами и UI элементами

Этот API позволяет Python модам создавать собственные окна с полным набором egui элементов.

---

## Управление окнами

### `create_window(id: str, title: str)`
Создаёт новое окно.

**Параметры:**
- `id` - уникальный идентификатор окна
- `title` - заголовок окна

**Пример:**
```python
text_engien.create_window("my_window", "My Custom Window")
```

### `show_window(id: str)`
Показывает скрытое окно.

### `hide_window(id: str)`
Скрывает окно.

### `set_window_position(id: str, x: float, y: float)`
Устанавливает позицию окна.

### `set_window_size(id: str, width: float, height: float)`
Устанавливает размер окна.

### `set_window_resizable(id: str, resizable: bool)`
Разрешает/запрещает изменение размера окна.

### `set_window_collapsible(id: str, collapsible: bool)`
Разрешает/запрещает сворачивание окна.

### `set_window_scroll(id: str, scroll: bool)`
Включает/выключает прокрутку содержимого окна.

---

## UI элементы

### Текстовые элементы

#### `add_label(window_id: str, text: str, wrap: bool = False, selectable: bool = False)`
Добавляет текстовую метку.

**Пример:**
```python
text_engien.add_label("my_window", "Hello, World!", True, False)
```

#### `add_heading(window_id: str, text: str, level: int = 1)`
Добавляет заголовок (уровни 1-3).

**Пример:**
```python
text_engien.add_heading("my_window", "Main Title", 1)
text_engien.add_heading("my_window", "Subtitle", 2)
```

#### `add_hyperlink(window_id: str, text: str, url: str)`
Добавляет кликабельную гиперссылку.

**Пример:**
```python
text_engien.add_hyperlink("my_window", "Visit GitHub", "https://github.com")
```

---

### Кнопки

#### `add_button(window_id: str, text: str, function_name: str = None, small: bool = False)`
Добавляет кнопку.

**Параметры:**
- `function_name` - имя Python функции для вызова при нажатии
- `small` - если True, создаёт маленькую кнопку

**Пример:**
```python
def on_click():
    print("Button clicked!")

text_engien.register_function("on_click", on_click)
text_engien.add_button("my_window", "Click Me!", "on_click", False)
```

---

### Поля ввода

#### `add_text_edit(window_id: str, id: str, multiline: bool = False, hint: str = None, password: bool = False)`
Добавляет поле ввода текста.

**Пример:**
```python
text_engien.add_text_edit("my_window", "username", False, "Enter username...", False)

# Получение значения
username = text_engien.get_text_value("my_window", "username")

# Установка значения
text_engien.set_text_value("my_window", "username", "John")
```

---

### Чекбоксы и радио-кнопки

#### `add_checkbox(window_id: str, id: str, label: str)`
Добавляет чекбокс.

**Пример:**
```python
text_engien.add_checkbox("my_window", "accept_terms", "I accept the terms")

# Получение значения
is_checked = text_engien.get_bool_value("my_window", "accept_terms")

# Установка значения
text_engien.set_bool_value("my_window", "accept_terms", True)
```

#### `add_radio_button(window_id: str, id: str, label: str, group: str)`
Добавляет радио-кнопку.

**Пример:**
```python
text_engien.add_radio_button("my_window", "option1", "Option 1", "my_group")
text_engien.add_radio_button("my_window", "option2", "Option 2", "my_group")
```

---

### Слайдеры и регуляторы

#### `add_slider(window_id: str, id: str, min: float, max: float, label: str = None, integer: bool = False)`
Добавляет слайдер.

**Пример:**
```python
text_engien.add_slider("my_window", "volume", 0.0, 100.0, "Volume", False)

# Получение значения
volume = text_engien.get_float_value("my_window", "volume")

# Установка значения
text_engien.set_float_value("my_window", "volume", 75.0)
```

#### `add_drag_value(window_id: str, id: str, label: str = None, speed: float = 1.0)`
Добавляет перетаскиваемое числовое значение.

**Пример:**
```python
text_engien.add_drag_value("my_window", "speed", "Speed", 0.1)
```

---

### Выпадающие списки

#### `add_combo_box(window_id: str, id: str, label: str, options: list[str])`
Добавляет выпадающий список.

**Пример:**
```python
text_engien.add_combo_box("my_window", "theme", "Select Theme", 
                         ["Light", "Dark", "Blue"])

# Получение индекса выбранного элемента
selected_index = text_engien.get_combo_value("my_window", "theme")

# Установка индекса
text_engien.set_combo_value("my_window", "theme", 1)  # Выбирает "Dark"
```

---

### Визуальные элементы

#### `add_separator(window_id: str)`
Добавляет разделительную линию.

#### `add_spacing(window_id: str, amount: float)`
Добавляет вертикальный отступ.

**Пример:**
```python
text_engien.add_separator("my_window")
text_engien.add_spacing("my_window", 20.0)
```

#### `add_progress_bar(window_id: str, id: str, show_percentage: bool = True)`
Добавляет прогресс-бар.

**Пример:**
```python
text_engien.add_progress_bar("my_window", "loading", True)
text_engien.set_float_value("my_window", "loading", 0.5)  # 50%
```

#### `add_spinner(window_id: str, size: float = 20.0)`
Добавляет анимированный индикатор загрузки.

**Пример:**
```python
text_engien.add_spinner("my_window", 25.0)
```

#### `add_image(window_id: str, path: str, width: float = None, height: float = None)`
Добавляет изображение.

**Пример:**
```python
text_engien.add_image("my_window", "path/to/image.png", 200.0, 150.0)
```

---

### Выбор цвета

#### `add_color_picker(window_id: str, id: str, label: str, alpha: bool = False)`
Добавляет выбор цвета.

**Параметры:**
- `alpha` - включает редактирование прозрачности

**Пример:**
```python
text_engien.add_color_picker("my_window", "bg_color", "Background Color", True)

# Получение цвета (список из 4 значений: R, G, B, A в диапазоне 0.0-1.0)
color = text_engien.get_color_value("my_window", "bg_color")

# Установка цвета
text_engien.set_color_value("my_window", "bg_color", [1.0, 0.0, 0.0, 1.0])  # Красный
```

---

## Полный пример

```python
import text_engien

def on_submit():
    # Получаем значения
    name = text_engien.get_text_value("form_window", "name")
    age = text_engien.get_float_value("form_window", "age")
    subscribe = text_engien.get_bool_value("form_window", "newsletter")
    theme = text_engien.get_combo_value("form_window", "theme")
    
    text_engien.print_to_console(f"Name: {name}")
    text_engien.print_to_console(f"Age: {age}")
    text_engien.print_to_console(f"Subscribe: {subscribe}")
    text_engien.print_to_console(f"Theme index: {theme}")

def create_form():
    # Создаём окно
    text_engien.create_window("form_window", "User Registration")
    text_engien.set_window_size("form_window", 400.0, 350.0)
    
    # Заголовок
    text_engien.add_heading("form_window", "Registration Form", 1)
    text_engien.add_separator("form_window")
    
    # Поля формы
    text_engien.add_label("form_window", "Name:", False, False)
    text_engien.add_text_edit("form_window", "name", False, "Enter your name", False)
    
    text_engien.add_spacing("form_window", 10.0)
    
    text_engien.add_label("form_window", "Age:", False, False)
    text_engien.add_slider("form_window", "age", 18.0, 100.0, "Age", True)
    
    text_engien.add_spacing("form_window", 10.0)
    
    text_engien.add_checkbox("form_window", "newsletter", "Subscribe to newsletter")
    
    text_engien.add_spacing("form_window", 10.0)
    
    text_engien.add_combo_box("form_window", "theme", "Preferred Theme",
                             ["Light", "Dark", "Auto"])
    
    text_engien.add_separator("form_window")
    text_engien.add_spacing("form_window", 10.0)
    
    # Кнопка отправки
    text_engien.add_button("form_window", "Submit", "on_submit", False)
    
    # Регистрируем функцию
    text_engien.register_function("on_submit", on_submit)

def run():
    text_engien.print_to_console("Creating form window...")
    create_form()
```

---

## Советы по использованию

1. **Уникальные ID**: Всегда используйте уникальные идентификаторы для окон и элементов
2. **Регистрация функций**: Не забывайте регистрировать функции обратного вызова через `register_function()`
3. **Состояние элементов**: Значения элементов сохраняются автоматически между кадрами
4. **Производительность**: Создавайте окна один раз при инициализации мода, а не в каждом кадре
5. **Типы данных**: 
   - Текстовые значения: `str`
   - Числовые значения: `float`
   - Логические значения: `bool`
   - Индексы комбобоксов: `int`
   - Цвета: `list[float]` из 4 элементов (RGBA)

---

## Совместимость с существующими функциями

Система окон полностью совместима с существующим API для вкладок и меню:

```python
# Старый API (вкладки и меню)
text_engien.register_tab("my_tab", "My Tab", "📁")
text_engien.add_dropdown_menu("my_tab", "File", "📄")

# Новый API (окна)
text_engien.create_window("my_window", "My Window")
```

Оба API могут использоваться одновременно в одном моде.