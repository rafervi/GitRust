use git2::{Repository, RemoteCallbacks, Cred};

fn main() {
    // Replace these values with your repository information
    let repo_path = "/path/to/your/repository";
    let remote_name = "origin";
    let branch_name = "main";
    let username = "your_username";
    let password = "your_password_or_personal_access_token";

    // Open the repository
    let repo = Repository::open(repo_path).expect("Failed to open repository");

    // Perform a Git pull
    pull(&repo, remote_name, branch_name, username, password);

    // Perform a Git push
    push(&repo, remote_name, branch_name, username, password);
}

fn pull(repo: &Repository, remote_name: &str, branch_name: &str, username: &str, password: &str) {
    // Fetch updates from the remote repository
    let mut remote = repo.find_remote(remote_name).expect("Failed to find remote");
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::userpass_plaintext(username_from_url.unwrap_or(username), password)
    });
    remote.fetch(&[branch_name], Some(&mut callbacks), None)
        .expect("Failed to fetch updates");

    // Merge the changes into the local branch
    let refspec = format!("refs/remotes/{}/{}:refs/heads/{}", remote_name, branch_name, branch_name);
    let refspec = refspec.as_str();
    let annotated_commit = repo.revparse_single(refspec).expect("Failed to parse refspec");
    let analysis = repo.merge_analysis(&[&annotated_commit])
        .expect("Failed to perform merge analysis");
    if analysis.0.is_up_to_date() {
        println!("Already up-to-date");
    } else {
        repo.merge(&[&annotated_commit], None, None)
            .expect("Failed to merge changes");
        println!("Successfully pulled changes");
    }
}

fn push(repo: &Repository, remote_name: &str, branch_name: &str, username: &str, password: &str) {
    // Push changes to the remote repository
    let mut remote = repo.find_remote(remote_name).expect("Failed to find remote");
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::userpass_plaintext(username_from_url.unwrap_or(username), password)
    });
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    let refspec = refspec.as_str();
    remote.push(&[refspec], Some(&mut callbacks))
        .expect("Failed to push changes");
    println!("Successfully pushed changes");
}