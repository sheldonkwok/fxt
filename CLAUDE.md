# Claude Development Notes

This file contains reminders and best practices for working on this codebase.

## Testing Workflow

**IMPORTANT: Always run tests before committing changes**

Proper workflow:
1. Make code changes
2. **Run `bun test` to verify nothing broke**
3. Review test results
4. Only commit if all tests pass
5. Push to remote

This applies to:
- Refactoring
- Bug fixes
- New features
- Any code modifications

## Test Commands

- Run all tests: `bun test`
- Test file location: `fixer.test.ts`

## Development Practices

- Keep functions focused and single-purpose
- Extract repeated logic into helper functions
- Preserve existing behavior when refactoring
- Verify tests pass after every change
