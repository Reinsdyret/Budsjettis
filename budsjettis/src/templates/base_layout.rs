use super::forms::{expense_form, expenses_section, charts_section};

pub fn base_layout() -> String {
    format!(r##"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Budsjettis</title>
        <script src="https://unpkg.com/htmx.org@1.9.10"></script>
        <script src="https://cdn.tailwindcss.com"></script>
        <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    </head>
    <body class="bg-gray-100 min-h-screen">
        <div class="max-w-4xl mx-auto p-6">
            <h1 class="text-3xl font-bold text-gray-800 mb-4">Velkommen til Budsjettis</h1>
            <p class="text-gray-600 mb-8">Budsjettis er en platform for å følge med på din egen pengebruk på din måte.</p>
            
            {}
            {}
            {}
        </div>
    </body>
    </html>
    "##, expense_form(), expenses_section(), charts_section())
}
