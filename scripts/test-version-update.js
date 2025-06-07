#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

/**
 * Test script to validate that our version update logic works correctly
 * This simulates what semantic-release will do when updating file versions
 */

function testVersionUpdate() {
    const testVersion = '2.0.0';
    
    console.log('Testing version update logic...');
    
    try {
        // Read current files
        const cargoPath = path.join(__dirname, '..', 'Cargo.toml');
        const extensionPath = path.join(__dirname, '..', 'extension.toml');
        
        const cargoContent = fs.readFileSync(cargoPath, 'utf8');
        const extensionContent = fs.readFileSync(extensionPath, 'utf8');
        
        console.log('Current Cargo.toml version:', cargoContent.match(/^version = "(.*)"/m)?.[1]);
        console.log('Current extension.toml version:', extensionContent.match(/^version = "(.*)"/m)?.[1]);
        
        // Test the update logic (same as in .releaserc.json)
        const updatedCargo = cargoContent.replace(/^version = ".*"/m, `version = "${testVersion}"`);
        const updatedExtension = extensionContent.replace(/^version = ".*"/m, `version = "${testVersion}"`);
        
        // Verify the replacements worked
        const cargoNewVersion = updatedCargo.match(/^version = "(.*)"/m)?.[1];
        const extensionNewVersion = updatedExtension.match(/^version = "(.*)"/m)?.[1];
        
        if (cargoNewVersion === testVersion && extensionNewVersion === testVersion) {
            console.log('✅ Version update logic works correctly!');
            console.log(`   Cargo.toml version would be updated to: ${cargoNewVersion}`);
            console.log(`   extension.toml version would be updated to: ${extensionNewVersion}`);
        } else {
            console.error('❌ Version update logic failed!');
            console.error(`   Expected: ${testVersion}`);
            console.error(`   Cargo.toml got: ${cargoNewVersion}`);
            console.error(`   extension.toml got: ${extensionNewVersion}`);
            process.exit(1);
        }
        
        // Show what the updated content would look like
        console.log('\nUpdated Cargo.toml would contain:');
        console.log(updatedCargo.split('\n').slice(0, 5).join('\n'));
        
        console.log('\nUpdated extension.toml would contain:');
        console.log(updatedExtension.split('\n').slice(0, 5).join('\n'));
        
    } catch (error) {
        console.error('❌ Test failed with error:', error.message);
        process.exit(1);
    }
}

if (require.main === module) {
    testVersionUpdate();
}

module.exports = { testVersionUpdate };