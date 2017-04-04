//! Access the Repos portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use util::url_join;
use client::{GetQueryBuilder, Executor};
use Json;

new_type!(Repos);
new_type!(Owner);
new_type!(Repo);

from!(GetQueryBuilder, Repos, "repos");
from!(Repos, Owner);
from!(Owner, Repo);
from!(Repo, Executor);

impl<'a> Repos<'a> {
    func!(owner, Owner, username_str);
}

impl<'a> Owner<'a> {
    func!(repo, Repo, repo_str);
}

impl<'a> Repo<'a> {
    exec!();
}
