use crate::models::expense::Expense;

#[derive(Debug)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub chart_type: ChartType,
    pub title: String,
}

#[derive(Debug)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
}

impl ChartType {
    fn to_string(&self) -> &str {
        match self {
            ChartType::Line => "line",
            ChartType::Bar => "bar", 
            ChartType::Pie => "pie",
        }
    }
}

pub fn expenses_by_date(expenses: &[Expense]) -> ChartData {
    use std::collections::HashMap;
    
    let mut daily_totals: HashMap<String, f64> = HashMap::new();
    
    for expense in expenses {
        let date = expense.date.split(' ').next().unwrap_or(&expense.date).to_string();
        *daily_totals.entry(date).or_insert(0.0) += expense.amount;
    }
    
    let mut entries: Vec<_> = daily_totals.into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    
    ChartData {
        labels: entries.iter().map(|(date, _)| date.clone()).collect(),
        values: entries.iter().map(|(_, amount)| *amount).collect(),
        chart_type: ChartType::Line,
        title: "Utgifter over tid".to_string(),
    }
}

pub fn expenses_by_category(expenses: &[Expense]) -> ChartData {
    use std::collections::HashMap;
    
    let mut category_totals: HashMap<String, f64> = HashMap::new();
    
    for expense in expenses {
        *category_totals.entry(expense.category.clone()).or_insert(0.0) += expense.amount;
    }
    
    ChartData {
        labels: category_totals.keys().cloned().collect(),
        values: category_totals.values().cloned().collect(),
        chart_type: ChartType::Pie,
        title: "Utgifter per kategori".to_string(),
    }
}

pub fn render_chart(chart_data: &ChartData, canvas_id: &str) -> String {
    let labels_json = chart_data.labels.iter()
        .map(|l| format!("'{}'", l))
        .collect::<Vec<_>>()
        .join(",");
    
    let values_json = chart_data.values.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");

    format!(r##"
    <div class="bg-white p-6 rounded-lg shadow">
        <h3 class="text-lg font-semibold mb-4">{}</h3>
        <canvas id="{}" width="400" height="200"></canvas>
    </div>
    <script>
        var ctx{};
        // Destroy existing Chart.js instance
        if (globalThis.budsjettisChart) {{
            globalThis.budsjettisChart.destroy();
        }} else {{
            ctx{} = document.getElementById('{}').getContext('2d');
        }}

        globalThis.budsjettisChart = new Chart(ctx{}, {{
            type: '{}',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Utgifter (kr)',
                    data: [{}],
                    borderColor: 'rgb(59, 130, 246)',
                    backgroundColor: [
                        'rgba(59, 130, 246, 0.6)',
                        'rgba(16, 185, 129, 0.6)', 
                        'rgba(245, 158, 11, 0.6)',
                        'rgba(239, 68, 68, 0.6)',
                        'rgba(139, 92, 246, 0.6)'
                    ],
                    tension: 0.1
                }}]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true
                    }}
                }}
            }}
        }});
    </script>
    "##, 
    chart_data.title,canvas_id, canvas_id,canvas_id ,canvas_id, canvas_id, 
    chart_data.chart_type.to_string(), labels_json, values_json)
}
