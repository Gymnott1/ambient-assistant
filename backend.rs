use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Suggestion {
    suggestion: String,
    command: String,
    comment: String,
}

#[derive(Debug, Clone, Serialize)]
struct Response {
    suggestions: Vec<Suggestion>,
    timestamp: u64,
}

type SuggestionStore = Arc<Mutex<Vec<Suggestion>>>;

#[tokio::main]
async fn main() {
    let suggestions: SuggestionStore = Arc::new(Mutex::new(vec![]));
    
    // 20 dummy suggestions
    let dummy_data = vec![
        Suggestion { suggestion: "🎵 Play Focus Music".to_string(), command: "spotify-play-focus".to_string(), comment: "Spotify • Lofi Beats".to_string() },
        Suggestion { suggestion: "📝 Git: Commit Changes".to_string(), command: "git-commit".to_string(), comment: "5 files modified".to_string() },
        Suggestion { suggestion: "🔑 SSH: Connect to Server".to_string(), command: "ssh-connect".to_string(), comment: "Root access ready".to_string() },
        Suggestion { suggestion: "🧹 Cleanup Downloads".to_string(), command: "cleanup-downloads".to_string(), comment: "1.2GB of temp files".to_string() },
        Suggestion { suggestion: "🚀 Deploy App".to_string(), command: "deploy-app".to_string(), comment: "All tests passed".to_string() },
        Suggestion { suggestion: "📊 Check System Stats".to_string(), command: "system-stats".to_string(), comment: "CPU: 45%, RAM: 60%".to_string() },
        Suggestion { suggestion: "🔄 Update Dependencies".to_string(), command: "update-deps".to_string(), comment: "12 packages available".to_string() },
        Suggestion { suggestion: "📧 Check Emails".to_string(), command: "check-email".to_string(), comment: "3 unread messages".to_string() },
        Suggestion { suggestion: "🌐 Open Dev Server".to_string(), command: "start-dev-server".to_string(), comment: "Port 3000 available".to_string() },
        Suggestion { suggestion: "📱 Build Mobile App".to_string(), command: "build-mobile".to_string(), comment: "React Native ready".to_string() },
        Suggestion { suggestion: "🔍 Run Tests".to_string(), command: "run-tests".to_string(), comment: "Jest • 45 test suites".to_string() },
        Suggestion { suggestion: "📦 Package Release".to_string(), command: "package-release".to_string(), comment: "Version 1.2.3 ready".to_string() },
        Suggestion { suggestion: "🎨 Design Review".to_string(), command: "design-review".to_string(), comment: "Figma • 3 components".to_string() },
        Suggestion { suggestion: "📈 Analytics Report".to_string(), command: "analytics-report".to_string(), comment: "Weekly insights ready".to_string() },
        Suggestion { suggestion: "🔐 Security Scan".to_string(), command: "security-scan".to_string(), comment: "No vulnerabilities found".to_string() },
        Suggestion { suggestion: "📝 Write Documentation".to_string(), command: "write-docs".to_string(), comment: "API endpoints updated".to_string() },
        Suggestion { suggestion: "🎯 Performance Audit".to_string(), command: "perf-audit".to_string(), comment: "Lighthouse score: 95".to_string() },
        Suggestion { suggestion: "🔧 Fix Bug #247".to_string(), command: "fix-bug-247".to_string(), comment: "Critical priority".to_string() },
        Suggestion { suggestion: "📋 Code Review".to_string(), command: "code-review".to_string(), comment: "3 PRs pending".to_string() },
        Suggestion { suggestion: "🎪 Demo Preparation".to_string(), command: "demo-prep".to_string(), comment: "Client meeting at 3pm".to_string() },
    ];
    
    *suggestions.lock().unwrap() = dummy_data;
    
    // Rotate suggestions every 3 seconds
    let suggestions_clone = suggestions.clone();
    tokio::spawn(async move {
        let mut index = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            let mut store = suggestions_clone.lock().unwrap();
            let total = store.len();
            
            // Rotate by moving first 5 to end
            let rotated: Vec<_> = store.iter().cycle().skip(index).take(total).cloned().collect();
            *store = rotated;
            
            index = (index + 5) % total;
            println!("🔄 Rotated suggestions, index: {}", index);
        }
    });
    
    let suggestions_filter = suggestions.clone();
    let api = warp::path("suggestions")
        .and(warp::get())
        .and(warp::any().map(move || suggestions_filter.clone()))
        .and_then(get_suggestions);
    
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET"]);
    
    println!("🚀 Backend running on http://localhost:8080/suggestions");
    
    warp::serve(api.with(cors))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn get_suggestions(suggestions: SuggestionStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = suggestions.lock().unwrap();
    let current_suggestions = store.iter().take(5).cloned().collect();
    
    let response = Response {
        suggestions: current_suggestions,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    
    Ok(warp::reply::json(&response))
}