#!/bin/bash

# Graff Release Script
# This script handles testing, building, linting, version management, and GitHub deployment

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check git status
check_git_status() {
    if ! git diff-index --quiet HEAD --; then
        print_error "You have uncommitted changes. Please commit or stash them before releasing."
        exit 1
    fi
    
    if ! git diff-index --cached --quiet HEAD --; then
        print_error "You have staged changes. Please commit them before releasing."
        exit 1
    fi
}

# Function to check if we're on the right branch
check_branch() {
    local current_branch=$(git branch --show-current)
    if [[ "$current_branch" != "main" && "$current_branch" != "master" ]]; then
        print_warning "You're not on main/master branch. Current branch: $current_branch"
        read -p "Do you want to continue? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Function to get current version
get_current_version() {
    grep '^version = ' Cargo.toml | cut -d'"' -f2
}

# Function to bump version
bump_version() {
    local current_version=$1
    local bump_type=$2
    
    # Parse current version
    local major=$(echo "$current_version" | cut -d'.' -f1)
    local minor=$(echo "$current_version" | cut -d'.' -f2)
    local patch=$(echo "$current_version" | cut -d'.' -f3 | cut -d'-' -f1)
    local prerelease=$(echo "$current_version" | cut -d'-' -f2-)
    
    case $bump_type in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            prerelease=""
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            prerelease=""
            ;;
        patch)
            patch=$((patch + 1))
            prerelease=""
            ;;
        *)
            print_error "Invalid bump type: $bump_type"
            exit 1
            ;;
    esac
    
    # Construct new version
    local new_version="$major.$minor.$patch"
    if [[ -n "$prerelease" ]]; then
        new_version="$new_version-$prerelease"
    fi
    
    echo "$new_version"
}

# Function to update version
update_version() {
    local new_version=$1
    local current_version=$(get_current_version)
    
    print_status "Updating version from $current_version to $new_version"
    
    # Update Cargo.toml
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    else
        # Linux
        sed -i "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    fi
    
    print_success "Version updated to $new_version"
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    
    if ! cargo test --release; then
        print_error "Tests failed!"
        exit 1
    fi
    
    print_success "All tests passed!"
}

# Function to run lints
run_lints() {
    print_status "Running lints..."
    
    # Check formatting
    if ! cargo fmt --all -- --check; then
        print_error "Code formatting check failed!"
        print_status "Run 'cargo fmt --all' to fix formatting issues."
        exit 1
    fi
    
    # Clippy checks
    if ! cargo clippy --release -- -D warnings; then
        print_error "Clippy checks failed!"
        exit 1
    fi
    
    print_success "All lints passed!"
}

# Function to build release
build_release() {
    print_status "Building release version..."
    
    if ! cargo build --release; then
        print_error "Build failed!"
        exit 1
    fi
    
    print_success "Release build successful!"
}

# Function to run cargo dist
run_cargo_dist() {
    print_status "Running cargo dist..."
    
    if ! cargo dist build --release; then
        print_error "Cargo dist build failed!"
        exit 1
    fi
    
    print_success "Cargo dist build successful!"
}

# Function to create git tag
create_tag() {
    local version=$1
    local tag_name="v$version"
    
    print_status "Creating git tag: $tag_name"
    
    if git tag -l | grep -q "^$tag_name$"; then
        print_error "Tag $tag_name already exists!"
        exit 1
    fi
    
    git tag -a "$tag_name" -m "Release $tag_name"
    print_success "Tag $tag_name created!"
}

# Function to push to GitHub
push_to_github() {
    local version=$1
    local tag_name="v$version"
    
    print_status "Pushing changes to GitHub..."
    
    # Push the main branch
    if ! git push origin HEAD; then
        print_error "Failed to push main branch!"
        exit 1
    fi
    
    # Push the tag
    if ! git push origin "$tag_name"; then
        print_error "Failed to push tag $tag_name!"
        exit 1
    fi
    
    print_success "Successfully pushed to GitHub!"
    print_success "Tag $tag_name is now available for cargo dist release!"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS] [version]"
    echo ""
    echo "Version Options (choose one):"
    echo "  --patch          Bump patch version (1.0.0 -> 1.0.1)"
    echo "  --minor          Bump minor version (1.0.0 -> 1.1.0)"
    echo "  --major          Bump major version (1.0.0 -> 2.0.0)"
    echo "  <version>        Specify exact version (e.g., 1.0.0)"
    echo ""
    echo "Other Options:"
    echo "  --skip-tests     Skip running tests"
    echo "  --skip-lints     Skip running lints"
    echo "  --skip-build     Skip building release"
    echo "  --skip-dist      Skip cargo dist build"
    echo "  --dry-run        Show what would be done without making changes"
    echo "  --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --patch                  # Bump patch version"
    echo "  $0 --minor                  # Bump minor version"
    echo "  $0 --major                  # Bump major version"
    echo "  $0 1.0.0                    # Specific version"
    echo "  $0 --patch --skip-tests     # Bump patch, skip tests"
    echo "  $0 --dry-run --minor        # See what minor bump would do"
    echo ""
    echo "Version format:"
    echo "  Use semantic versioning (e.g., 1.0.0, 1.2.3, 2.0.0-beta.1)"
}

# Function to validate version format
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$ ]]; then
        print_error "Invalid version format: $version"
        print_error "Use semantic versioning (e.g., 1.0.0, 1.2.3, 2.0.0-beta.1)"
        exit 1
    fi
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if cargo is installed
    if ! command_exists cargo; then
        print_error "Cargo is not installed!"
        exit 1
    fi
    
    # Check if git is installed
    if ! command_exists git; then
        print_error "Git is not installed!"
        exit 1
    fi
    
    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Not in a git repository!"
        exit 1
    fi
    
    print_success "All prerequisites met!"
}

# Main script
main() {
    local skip_tests=false
    local skip_lints=false
    local skip_build=false
    local skip_dist=false
    local dry_run=false
    local version=""
    local bump_type=""
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --skip-tests)
                skip_tests=true
                shift
                ;;
            --skip-lints)
                skip_lints=true
                shift
                ;;
            --skip-build)
                skip_build=true
                shift
                ;;
            --skip-dist)
                skip_dist=true
                shift
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --patch)
                bump_type="patch"
                shift
                ;;
            --minor)
                bump_type="minor"
                shift
                ;;
            --major)
                bump_type="major"
                shift
                ;;
            --help)
                show_usage
                exit 0
                ;;
            -*)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
            *)
                if [[ -z "$version" ]]; then
                    version=$1
                else
                    print_error "Multiple versions specified: $version and $1"
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # Handle version specification
    if [[ -n "$bump_type" ]]; then
        # Auto-bump version
        local current_version=$(get_current_version)
        version=$(bump_version "$current_version" "$bump_type")
        print_status "Auto-bumping version from $current_version to $version ($bump_type)"
    elif [[ -z "$version" ]]; then
        print_error "No version specified! Use --patch, --minor, --major, or provide a specific version."
        show_usage
        exit 1
    else
        # Manual version provided
        validate_version "$version"
    fi
    
    # Show what we're going to do
    print_status "Starting release process for version $version"
    print_status "Current version: $(get_current_version)"
    
    if [[ "$dry_run" == true ]]; then
        print_warning "DRY RUN MODE - No changes will be made"
        echo ""
        echo "Would perform the following actions:"
        echo "1. Check git status and branch"
        echo "2. Check prerequisites"
        if [[ "$skip_tests" == false ]]; then
            echo "3. Run tests"
        fi
        if [[ "$skip_lints" == false ]]; then
            echo "4. Run lints"
        fi
        if [[ "$skip_build" == false ]]; then
            echo "5. Build release"
        fi
        if [[ "$skip_dist" == false ]]; then
            echo "6. Run cargo dist build"
        fi
        echo "7. Update version to $version"
        echo "8. Commit changes"
        echo "9. Create tag v$version"
        echo "10. Push to GitHub"
        exit 0
    fi
    
    # Check prerequisites
    check_prerequisites
    
    # Check git status
    check_git_status
    check_branch
    
    # Run tests
    if [[ "$skip_tests" == false ]]; then
        run_tests
    else
        print_warning "Skipping tests"
    fi
    
    # Run lints
    if [[ "$skip_lints" == false ]]; then
        run_lints
    else
        print_warning "Skipping lints"
    fi
    
    # Build release
    if [[ "$skip_build" == false ]]; then
        build_release
    else
        print_warning "Skipping build"
    fi
    
    # Run cargo dist
    if [[ "$skip_dist" == false ]]; then
        run_cargo_dist
    else
        print_warning "Skipping cargo dist"
    fi
    
    # Update version
    update_version "$version"
    
    # Commit changes
    print_status "Committing version change..."
    git add Cargo.toml
    git commit -m "Bump version to $version"
    print_success "Version change committed!"
    
    # Create tag
    create_tag "$version"
    
    # Push to GitHub
    push_to_github "$version"
    
    # Final success message
    echo ""
    print_success "🎉 Release $version completed successfully!"
    print_status "The release is now available on GitHub and ready for distribution."
    print_status "You can monitor the release process in your GitHub repository."
}

# Run main function with all arguments
main "$@"
