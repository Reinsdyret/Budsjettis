pub fn expense_form() -> &'static str {
    r##"
    <div class="bg-white p-6 rounded-lg shadow mb-6">
        <h2 class="text-xl font-semibold mb-4">Legg til ny utgift</h2>
        <form hx-post="/expenses" hx-target="#form-result" class="space-y-4">
            <div>
                <label class="block text-sm font-medium text-gray-700">Beskrivelse</label>
                <input type="text" name="description" required 
                       class="mt-1 block w-full rounded-md border-gray-300 shadow-sm p-2 border">
            </div>
            <div>
                <label class="block text-sm font-medium text-gray-700">Beløp (kr)</label>
                <input type="number" step="0.01" name="amount" required 
                       class="mt-1 block w-full rounded-md border-gray-300 shadow-sm p-2 border">
            </div>
            <div>
                <label class="block text-sm font-medium text-gray-700">Kategori</label>
                <select name="category" required 
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm p-2 border">
                    <option value="food">Mat</option>
                    <option value="transport">Transport</option>
                    <option value="entertainment">Underholdning</option>
                    <option value="shopping">Shopping</option>
                    <option value="other">Annet</option>
                </select>
            </div>
            <button type="submit" 
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                Legg til utgift
            </button>
        </form>
        <div id="form-result" class="mt-4"></div>
    </div>
    "##
}

pub fn expenses_section() -> &'static str {
    r##"
    <div class="bg-white p-6 rounded-lg shadow">
        <h2 class="text-xl font-semibold mb-4">Dine utgifter</h2>
        <button hx-get="/expenses" hx-target="#expenses-list"
                class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded mb-4">
            Last inn utgifter
        </button>
        <div id="expenses-list" class="max-w-sm"></div>
    </div>
    "##
}
